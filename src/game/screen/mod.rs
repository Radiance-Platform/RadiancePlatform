use std::fmt::Debug;
use crate::game::config_parsers::GameData;
use crate::game::GameState;

use std::io::{Error, ErrorKind, stdout};
use std::process::{exit};

use crossterm::{
    execute,
    style::{Print},
    ExecutableCommand, Result,
    terminal::{SetSize, size},
    cursor::{MoveTo}
};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crate::game::maps::MapData;
use crate::game::characters::Character;
use crate::game::characters::attribute;
use super::objects::{ObjectInteraction, Object};

#[derive(Clone, Debug)]
pub enum VisualState {
    StartScreen,
    PlayingMap,
    PlayingDialog,
    PlayingInventory,
    PlayingCharacterInteraction,
    PlayingCharacterFight,
}

#[derive(Clone, Debug)]
pub struct Screen {
    original_columns: u16,
    original_rows: u16,
    current_columns: u16,
    current_rows: u16,
}

impl Screen {

    // Throw an error if the terminal size is too small to run the game
    //      (Terminal must be at least 80x20)
    fn check_screen_size() -> Result<()> {
        let error_terminal_too_small = Error::new(ErrorKind::Other, "Terminal size is too small, must be at least 80x20");
        let (cols, rows) = size()?;
        return if cols < 80 || rows < 20 {
            Err(error_terminal_too_small)
        } else {
            Ok(())
        }
    }

    // Set up the screen to display the game
    pub fn initialize() -> Screen {
        // Get the original size
        let (orig_cols, orig_rows) = size().unwrap();

        // Make sure their terminal is big enough
        match Screen::check_screen_size() {
            Err(_) => {
                println!("ERROR: Screen size ({}x{}) is too small, please use a larger terminal.",
                         orig_cols, orig_rows);
                exit(1);
            },
            Ok(_) => {
                println!("Screen size check passed");
            }
        }

        // Store original and desired sizes in a screen struct
        let screen = Screen {
            original_columns: orig_cols,
            original_rows: orig_rows,
            current_columns: 80,
            current_rows: 20
        };

        // Turn off the cursor
        // TODO: Make a match error check like those other ones
        match stdout().execute(Hide) {
            Err(_) => {
                println!("ERROR: Unable to hide the terminal cursor, please try another terminal");
                exit(1);
            },
            _ => {
                println!("Terminal cursor hidden");
            }
        };

        // Turn on raw mode for proper keyboard input access
        match enable_raw_mode() {
            Err(_) => {
                println!("ERROR: Unable to enable raw terminal mode for input, please try another terminal");
                exit(1);
            },
            _ => {
                println!("Raw terminal mode enabled");
            }
        };

        // Set the size
        match execute!(stdout(), SetSize(screen.current_columns, screen.current_rows)) {
            Err(_) => {
                println!("ERROR: Failed to set the screen size");
                exit(1);
            },
            _ => {
                println!("Screen size set");
            }
        };

        return screen;

    }

    // Function to take in a string and determine what column it should start being printed at
    // in order to horizontally center it. `s` should be the string to center.
    fn horizontally_center_start_position(&self, s: &str, container_cols: u16) -> u16 {
        if container_cols < s.len() as u16 { return 0; } // if there is overflow
        let empty_space = container_cols - (s.len() as u16);
        return empty_space/2;
    }

    // Function to take in a number of lines and determine what row (line) they should start
    // being printed at in order to vertically center them.
    fn vertically_center_start_position(&self, c: u16, container_rows: u16) -> u16 {
        if container_rows < c { return 0; } // if there is overflow
        let empty_space = container_rows - c;
        return empty_space/2;
    }

    // Test function to use for making sure I can blink the cursor like when a character is over
    // another object
    fn blink_player_cursor(&self, game_data: &GameData, game_state: &mut GameState, start_x: u16, start_y: u16, x: u16, y: u16) -> Result<()> {
        stdout().execute(MoveTo(start_x+x, start_y+y))?;
        if game_state.cursor_blink {
            // Print character
            stdout().execute(Print(game_data.info.player.as_ref().unwrap().icon))?;
        } else {
            // Print whatever map thing is there
            if game_data.maps[game_state.current_map].grid[x as usize][y as usize].is_some() {
                match game_data.maps[game_state.current_map].grid[x as usize][y as usize].as_ref().unwrap()  {
                    MapData::Character(character) => { stdout().execute(Print(character.icon))?; }
                    MapData::Object(object) => { stdout().execute(Print(object.icon))?; }
                }
            }
        }

        game_state.cursor_blink = !game_state.cursor_blink;

        Ok(())
    }

    // Function for handling the exit dialog and action
    fn handle_exit_key(&self, game_state: &mut GameState) {
        game_state.dialog_message = "Would you like to exit the game?".to_string();
        game_state.dialog_option_0 = "No".to_string();
        game_state.dialog_option_1 = "Yes".to_string();
        game_state.dialog_return_0 = game_state.visual_state.clone();
        game_state.dialog_return_cancel = game_state.visual_state.clone();
        game_state.visual_state = VisualState::PlayingDialog;
        game_state.pre_exit = true;
    }

    // Function for handling object interactions (when the "interact" key is pressed)
    fn handle_interact_key(&self, game_state: &mut GameState, game_data: &mut GameData) {
        let map = &game_data.maps[game_state.current_map].clone();
        let player_x = game_state.current_player_x as usize;
        let player_y = game_state.current_player_y as usize;
        if map.grid[player_x][player_y].is_some() {
            match map.grid[player_x][player_y].as_ref().unwrap()  {
                MapData::Character(character) => {
                    self.character_interact(game_state, character);
                }
                MapData::Object(object) => {
                    self.activate_object(game_state, game_data, object);
                }
            }
        }
    }

    // Starts any interaction that happens when an npc conversation is started
    //    with the interact key
    fn character_interact(&self, game_state: &mut GameState, character: &Character) {
        game_state.dialog_return_cancel = game_state.visual_state.clone();
        game_state.visual_state = VisualState::PlayingCharacterInteraction;
        game_state.npc_dialog_id = character.dialog_id.clone();
    }

    // Starts any interaction that happens when an object is activated with the interact key
    fn activate_object(&self, game_state: &mut GameState, game_data: &mut GameData, object: &Object) {
        match object.category.as_str() {
            "collectable" => {
                // Remove item from map and add to inventory
                self.collect_object(game_state, game_data, object);
                // display "found item" dialog
                game_state.dialog_message = format!("You've found the {}!\n\nNow, what will you do with it?"
                                                    , object.name);
                game_state.dialog_option_0 = "Open inventory".to_string();
                game_state.dialog_option_1 = "Close".to_string();
                game_state.dialog_return_0 = VisualState::PlayingInventory;
                game_state.dialog_return_1 = game_state.visual_state.clone();
                game_state.dialog_return_cancel = game_state.visual_state.clone();
                game_state.pre_exit = false;
                game_state.visual_state = VisualState::PlayingDialog;
                return;
            }
            _ => {
            }
        }
        for interaction in &object.interactions {
            match interaction {
                ObjectInteraction::ObjectInteractionActivate(activate) => {
                    if !object.prereqs_met(&activate.prereqs) {
                        // TODO: Add a more personalized dialog box here.
                        //   Specify in config file?
                        if object.category == "door" {
                            // Display dialog for door being locked
                            game_state.dialog_message = "The door is locked! Try to find a key.".to_string();
                            game_state.dialog_option_0 = "Open inventory".to_string();
                            game_state.dialog_option_1 = "Close".to_string();
                            game_state.dialog_return_0 = VisualState::PlayingInventory;
                            game_state.dialog_return_1 = game_state.visual_state.clone();
                            game_state.dialog_return_cancel = game_state.visual_state.clone();
                            game_state.pre_exit = false;
                            game_state.visual_state = VisualState::PlayingDialog;
                        }
                        continue;
                    }
                    if activate.category == "travel" {
                        self.travel_through_door(game_state, game_data, object);
                    }
                }
                ObjectInteraction::ObjectInteractionObjectUse(_object_use) => {
                }
            }
        }
    }

    // Uses the object currently selected in the inventory on the object currently under the player.
    fn use_object(&self, game_state: &mut GameState, game_data: &mut GameData) {
        let x = game_state.current_player_x as usize;
        let y = game_state.current_player_y as usize;
        let i_x = game_state.inventory_x;
        let i_y = game_state.inventory_y;

        // Get the object on the map to use the item on
        let map_object;
        let map = game_data.maps[game_state.current_map].clone();
        if map.grid[x][y].is_some() {
            match map.grid[x][y].as_ref().unwrap()  {
                MapData::Character(character) => {
                    self.use_object_character(game_state, game_data, character);
                    return;
                }
                MapData::Object(object) => { map_object = object; }
            }
        } else {
            // Nothing to use the object on. Display dialog and do nothing.
            self.show_game_message(game_state, format!("You can't use this item here!"));
            return;
        }

        // Get the selected inventory object
        let inventory_object;
        let inventory = game_data.info.player.as_ref().unwrap().inventory.clone();
        if inventory[i_x][i_y].is_some() {
            inventory_object = inventory[i_x][i_y].as_ref().unwrap();
        } else {
            return; // Nothing in inventory slot. This shouldn't really happen at this point.
        }

        let mut object_used = false;
        // find object use interaction
        for interaction in &map_object.interactions {
            match interaction {
                ObjectInteraction::ObjectInteractionActivate(_activate) => {}
                ObjectInteraction::ObjectInteractionObjectUse(object_use) => {
                    if object_use.foreign_object_id != inventory_object.id {
                        continue;
                    }
                    // The item was used. Display message.
                    self.show_game_message(game_state, format!("You used the {} on the {}!",
                                                    inventory_object.name, map_object.name));
                    // Go through each action in the interaction
                    for action in &object_use.self_action {
                        // Perform all self-actions
                        object_used = true;
                        let mut new_map_object = map_object.clone();
                        new_map_object.set_state(action.name.clone(), action.value);
                        game_data.maps[game_state.current_map]
                                .grid[game_state.current_player_x as usize][game_state.current_player_y as usize]
                                = Option::<MapData>::Some(MapData::Object(new_map_object));
                    }
                    // If the item is a door, perform all the actions on the other side of the door.
                    if map_object.category == "door" {
                        // Get position of other door
                        let (mut door_map, mut door_x, mut door_y) = (0, 0, 0);
                        self.get_door_other_side(game_state, game_data, map_object.id.clone(),
                                                 &mut door_map, &mut door_x, &mut door_y);
                        // Perform actions on other door
                        for action in &object_use.self_action {
                            // Perform all self-actions
                            let mut new_door = map_object.clone();
                            new_door.set_state(action.name.clone(), action.value);
                            game_data.maps[door_map]
                                    .grid[door_x as usize][door_y as usize]
                                    = Option::<MapData>::Some(MapData::Object(new_door));
                        }
                    }
                    // If the item is consumed, remove it from the inventory
                    if object_use.consume_item {
                        let mut new_player = game_data.info.player.as_ref().unwrap().clone();
                        new_player.inventory[i_x][i_y] = Option::None;
                        game_data.info.player = Option::<Character>::Some(new_player);
                    }
                }
            }
        }
        if !object_used {
            let message = format!("You tried to use the {} on the {}, but it didn't work!",
                                                inventory_object.name, map_object.name);
            self.show_game_message(game_state, message);
        }
        return;
    }

    // Uses an object from the player's inventory on a character in the
    //    player's spot on the map
    fn use_object_character(&self, game_state: &mut GameState, game_data: &mut GameData, character: &Character) {
        let i_x = game_state.inventory_x;
        let i_y = game_state.inventory_y;

        // Get the selected inventory object
        let inventory_object;
        let inventory = game_data.info.player.as_ref().unwrap().inventory.clone();
        if inventory[i_x][i_y].is_some() {
            inventory_object = inventory[i_x][i_y].as_ref().unwrap();
        } else {
            return; // Nothing in inventory slot. This shouldn't really happen at this point.
        }

        // Find the right interaction for the item
        let mut object_used = false;
        for object_use in &character.interactions.object_use {
            if object_use.object_id == inventory_object.id {
                object_used = true;

                // Update character dialog if specified by the interaction
                if !object_use.set_dialog.is_empty() {
                    let mut new_character = character.to_owned();
                    new_character.dialog_id = object_use.set_dialog.to_owned();
                    game_data.maps[game_state.current_map]
                            .grid[game_state.current_player_x as usize][game_state.current_player_y as usize]
                            = Option::<MapData>::Some(MapData::Character(new_character));
                }

                // If the item is consumed, remove it from the inventory
                if object_use.consume_item {
                    let mut new_player = game_data.info.player.as_ref().unwrap().clone();
                    new_player.inventory[i_x][i_y] = Option::None;
                    game_data.info.player = Option::<Character>::Some(new_player);
                }
            }
        }
        if !object_used {
            self.show_game_message(game_state, format!("You can't use this item here!"));
        }
    }

    // Removes the object from the player's spot on the map and places it in the
    //     player inventory
    fn collect_object(&self, game_state: &mut GameState, game_data: &mut GameData, object: &Object) {
        // If inventory size is not exceeded, add item to player inventory
        if game_data.info.player.is_none() {
            return;
        }
        let mut player = game_data.info.player.as_ref().unwrap().clone();
        player.collect_object(object);
        game_data.info.player = Option::<Character>::Some(player);
        // Remove item from map
        game_data.maps[game_state.current_map]
                 .grid[game_state.current_player_x as usize][game_state.current_player_y as usize]
                 = Option::None;
    }

    // Moves character to a different map through the specified door
    // Assume prereqs are already checked.
    fn travel_through_door(&self, game_state: &mut GameState, game_data: &GameData, door: &Object) {
        // find door
        let (mut m, mut x, mut y) = (0, 0, 0);
        self.get_door_other_side(game_state, game_data, door.id.clone(),
                                &mut m, &mut x, &mut y);
        // Move character to new door
        game_state.current_map = m;
        game_state.current_player_x = x as u16;
        game_state.current_player_y = y as u16;
    }

    // Sets door_map, x, and y to the position of the door with door_id that is not in current space.
    fn get_door_other_side(&self, game_state: &GameState, game_data: &GameData, door_id: String,
                           door_map: &mut usize, x: &mut usize, y: &mut usize) {
        *door_map = 0;
        *x = 0;
        *y = 0;
        // Go through each map
        for m in 0..game_data.maps.len() {
            let map = &game_data.maps[m];
            // Door should not be in the same map as the current map
            if m == game_state.current_map {
                continue;
            }
            // Look at every square in the map
            for c in 0..map.grid.len() {
                // Then by each row
                for r in 0..map.grid[c].len() {
                    if map.grid[c][r].is_some() {
                        match map.grid[c][r].as_ref().unwrap()  {
                            MapData::Character(_character) => {
                            }
                            MapData::Object(object) => {
                                if object.id == door_id { // If correct door is found
                                    // Move character to new door
                                    *door_map = m;
                                    *x = c;
                                    *y = r;
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Sets the dialog screen to display a popup with the specified msg string.
    //    The popup closes upon clicking return and returns to the current
    //    screen.
    fn show_game_message(&self, game_state: &mut GameState, msg: String) {
            game_state.dialog_message = msg;
            game_state.dialog_option_0 = "Continue".to_string();
            game_state.dialog_option_1 = "Close".to_string();
            game_state.dialog_return_0 = game_state.visual_state.clone();
            game_state.dialog_return_1 = game_state.visual_state.clone();
            game_state.dialog_return_cancel = game_state.visual_state.clone();
            game_state.pre_exit = false;
            game_state.visual_state = VisualState::PlayingDialog;
    }

    // Draws a rectangular border with given start coordinates and width/heights
    fn draw_border(&self, start_col: u16, start_row: u16, cols: u16, rows: u16) -> Result<()> {
        // Loop over each row
        for r in start_row..start_row+rows {
            // Loop over each column
            stdout().execute(MoveTo(start_col, r))?;
            for c in start_col..start_col+cols {
                if r == start_row || r == start_row+rows-1 {
                    if c == start_col || c == start_col+cols-1 {
                        stdout().execute(Print("+"))?;
                    } else {
                        stdout().execute(Print("-"))?;
                    }
                } else {
                    if c == start_col || c == start_col+cols-1 {
                        stdout().execute(Print("|"))?;
                    } else {
                        stdout().execute(Print(" "))?;
                    }
                }
            }
        }

        Ok(())
    }

    // Draws a rectangular border with given start coordinates and width/heights.
    // Adds a highlight ('=' signs around the inside of the border) to indicate the box is selected.
    fn draw_highlight_border(&self, start_col: u16, start_row: u16, cols: u16, rows: u16) -> Result<()> {
        self.draw_border(start_col, start_row, cols, rows)?;
        // Loop over each row
        let h_start_col = start_col+1;
        let h_start_row = start_row+1;
        let h_cols = cols-2;
        let h_rows = rows-2;
        for r in h_start_row..h_start_row+h_rows {
            // Loop over each column
            stdout().execute(MoveTo(h_start_col, r))?;
            for c in h_start_col..h_start_col+h_cols {
                if r == h_start_row || r == h_start_row+h_rows-1 ||
                   c == h_start_col || c == h_start_col+h_cols-1 {
                    stdout().execute(Print("="))?;
                } else {
                    stdout().execute(Print(" "))?;
                }
            }
        }

        Ok(())
    }

    // Draws a rectangular border with given start coordinates and width/heights, then adds
    //    text centered in the box.
    // If highlight is true, adds a highlight ('=' signs around the inside of the border) to
    //    indicate the box is selected.
    fn draw_text_box(&self, start_col: u16, start_row: u16, cols: u16, rows: u16, text: &String, highlight: bool) -> Result<()> {
        if highlight {
            self.draw_highlight_border(start_col, start_row, cols, rows)?;
        } else {
            self.draw_border(start_col, start_row, cols, rows)?;
        }
        let text_rows = rows - 4;
        let text_cols = cols - 4;
        let text_start_col = start_col + 2;
        let text_start_row = start_row + 2;
        let lines = textwrap::wrap(&text, text_cols as usize);
        let mut row = text_start_row +
                           self.vertically_center_start_position(lines.len() as u16, text_rows);
        for line in lines {
            execute!(
                stdout(),
                MoveTo(self.horizontally_center_start_position(&line, text_cols) + text_start_col, row),
                Print(line),
            )?;
            row += 1;
        }

        Ok(())
    }

    fn draw_item_grid(&self, items: &Vec<Vec<Option<Object>>>, start_col: u16, start_row: u16,
                      selected_col: usize, selected_row: usize) -> Result<()> {
        let box_cols: u16 = 18;
        let mut box_rows: u16 = 7;

        for c in 0..items.len() {
            for r in 0..items[c].len() {
                // Draw the box for this array spot
                let box_start_col = start_col + (c as u16)*(box_cols-1);
                let box_start_row = start_row + (r as u16)*(box_rows-1);
                if r == 2 { // Temporary fix to make the items fit perfectly in the screen
                    box_rows += 1;
                }
                if selected_col == c && selected_row == r {
                    self.draw_highlight_border(box_start_col, box_start_row, box_cols, box_rows)?;
                } else {
                    self.draw_border(box_start_col, box_start_row, box_cols, box_rows)?;
                }
                if items[c][r].is_some() {
                    let item = items[c][r].as_ref().unwrap();
                    // display item name and icon
                    let name = item.name.clone();
                    let icon = item.icon;
                    let name_start_col = box_start_col
                                              + self.horizontally_center_start_position(&name, box_cols);
                    let icon_start_col = box_start_col
                                              + self.horizontally_center_start_position("i", box_cols);
                    let icon_start_row = box_start_row
                                              + self.vertically_center_start_position(1, box_rows);
                    stdout().execute(MoveTo(name_start_col, box_start_row+2))?;
                    stdout().execute(Print(name))?;
                    stdout().execute(MoveTo(icon_start_col, icon_start_row))?;
                    stdout().execute(Print(icon))?;

                    let use_start_col = box_start_col
                                            + self.horizontally_center_start_position("Use?", box_cols);
                    if r == selected_row && c == selected_col {
                        stdout().execute(MoveTo(use_start_col, box_start_row + box_rows - 2))?;
                        stdout().execute(Print("Use?"))?;
                    }
                }
                if r == 2 { // Temporary fix to make the items fit perfectly in the screen
                    box_rows -= 1;
                }
            }
        }

        Ok(())
    }

    // Draw stats (attributes) in a list with the top right corner at (start_col, start_row).
    //    Stats are in format: <stat name>: <current_value>/<max_value>
    fn draw_stat_display(&self, stats: &Vec<attribute::Attribute>, start_col: u16, start_row: u16) -> Result<()> {
        let mut row = 0;
        for stat in stats {
            // TODO: Figure out stat bar formatting
            let line = format!("{}: {}/{}", stat.display_name, stat.current_val, stat.max_val);
            execute!(
                stdout(),
                MoveTo(start_col, start_row + row),
                Print(line),
            )?;
            row += 1;
        }

        Ok(())
    }

    fn draw_face(&self, start_col: u16, start_row: u16, character: &Character) -> Result<()> {
        let cols = 18;
        let rows = 7;
        self.draw_border(start_col, start_row, cols, rows)?;
        if character.id == "player" {
            execute!(
                stdout(),
                MoveTo(start_col+5, start_row + 1),
                Print(".       ."),
                MoveTo(start_col+3, start_row + 4),
                Print("|          |"),
                MoveTo(start_col+3, start_row + 5),
                Print("+----------+"),
                MoveTo(start_col + self.horizontally_center_start_position(&character.name, cols), start_row+rows+1),
                Print(&character.name),
            )?;
        } else {
            execute!(
                stdout(),
                MoveTo(start_col+5, start_row + 1),
                Print("v       v"),
                MoveTo(start_col+3, start_row + 4),
                Print("+----------+"),
                MoveTo(start_col+3, start_row + 5),
                Print("|          |"),
                MoveTo(start_col + self.horizontally_center_start_position(&character.name, cols), start_row+rows+1),
                Print(&character.name),
            )?;
        }

        Ok(())
    }

    // Draws the start screen with the game name, author, description, and instructions for how to play
    fn draw_start_screen(&self, game_data: &mut GameData, game_state: &mut GameState) -> Result<()> {
        self.draw_border(0, 0, 80, 20)?;
        // Print game info
        let mut lines = Vec::<&str>::new();
        let name_line = format!("Welcome to {}", game_data.info.name);
        let author_line = format!("Written by {}", game_data.info.author);
        let description_lines = textwrap::wrap(&game_data.info.description, (self.current_columns - 12) as usize);

        lines.push(&name_line);
        lines.push(&author_line);
        lines.push("Powered by the Radiance Platform");
        lines.push("");

        for l in &description_lines {
            lines.push(l.as_ref());
        }

        lines.push("");
        lines.push("Controls:");
        lines.push("Use WASD or the Arrow Keys to move around the world.");
        lines.push("Use Enter to interact with objects in the world.");
        lines.push("Use E to open your inventory, WASD/Arrows");
        lines.push("to move within, and Enter to select items.");
        lines.push("Press Enter to select options within dialog boxes.");
        lines.push("");
        lines.push("");
        lines.push("Press Enter to start the game.");

        let mut row = self.vertically_center_start_position(lines.len() as u16, self.current_rows);
        for line in lines {
            execute!(
                stdout(),
                MoveTo(self.horizontally_center_start_position(line, self.current_columns), row),
                Print(line),
            )?;
            row += 1;
        }

        if !game_state.last_character_processed {

            let keycode = match game_state.last_character_pressed.as_ref().unwrap() {
                Event::Key(x) => {
                    x.code
                },
                _ => { KeyCode::Null }
            };

            // Process exiting the game
            // TODO: Add processing for other key presses here
            // For example, control+C via something like
            // game_state.last_character_pressed.as_ref().unwrap().modifiers == KeyModifiers::CONTROL && keycode == KeyCode::Char('C')
            if keycode == KeyCode::Esc {
                self.handle_exit_key(game_state);

            } else if keycode == KeyCode::Enter {
                // Change to map view
                game_state.visual_state = VisualState::PlayingMap;
            }

            game_state.last_character_processed = true;
            match self.draw(game_data, game_state) {
                Ok(_) => {},
                Err(_) => {
                    println!("ERROR: Problem encountered while drawing screen, exiting!");
                    self.end()?;
                }
            }
        }

        Ok(())
    }

    // Draws the playing map (main gameplay screen) and handles keypress input for moving the character,
    //      item interaction, and changing screens.
    fn draw_playing_map(&self, game_data: &mut GameData, game_state: &mut GameState) -> Result<()> {

        // Clear the screen
        self.draw_border(0, 0, 80, 20)?;

        // Find the current map that the player is in
        let map = &game_data.maps[game_state.current_map];

        // Create a wrapped version of the map's description
        let description = textwrap::wrap(&map.info.description, 80-4);

        // Draw box at the top
        self.draw_border(0, 0, 80, 2+description.len() as u16)?;

        // Draw the map room description
        for i in 0..description.len() {
            execute!(
                stdout(),
                MoveTo(2, 1+i as u16),
                Print(&description[i]),
            )?;
        }

        // Draw the map room itself

        let start_c = 8;
        let start_r = 6;

        // Border first
        self.draw_border(start_c, start_r, map.grid.len() as u16, map.grid[0].len() as u16)?;

        // Then items
        // Go first by each column
        for c in 0..map.grid.len() {
            // Then by each row
            for r in 0..map.grid[c].len() {
                if map.grid[c][r].is_some() {
                    match map.grid[c][r].as_ref().unwrap()  {
                        MapData::Character(character) => {
                            execute!(
                                stdout(),
                                MoveTo(start_c+c as u16, start_r+r as u16),
                                Print(character.icon),
                            )?;
                        }
                        MapData::Object(object) => {
                            execute!(
                                stdout(),
                                MoveTo(start_c+c as u16, start_r+r as u16),
                                Print(object.icon),
                            )?;
                        }
                    }
                }
            }
        }

        // Then the player
        self.blink_player_cursor(game_data, game_state, start_c, start_r,
                                 game_state.current_player_x,
                                 game_state.current_player_y)?;


        // Process any input the player provided
        if !game_state.last_character_processed {

            let keycode = match game_state.last_character_pressed.as_ref().unwrap() {
                Event::Key(x) => {
                    x.code
                },
                _ => { KeyCode::Null }
            };

            if keycode == KeyCode::Esc {
                // Process exiting the game
                self.handle_exit_key(game_state);

            } else if keycode == KeyCode::Char('M') {
                // Change to the next map
                if game_state.current_map + 1 <  game_data.maps.len() {
                    game_state.current_map = game_state.current_map + 1;
                } else {
                    game_state.current_map = 0;
                }

            } else if keycode == KeyCode::Char('H') {
                // Change to home view
                game_state.visual_state = VisualState::StartScreen;
                execute!(
                        stdout(),
                        MoveTo(10, 11),
                        Print("Changing to start screen")
                    )?;

            } else if keycode == KeyCode::Up || keycode == KeyCode::Char('w') {
                // Handle moving the player upward
                if self.check_move_available(game_data, game_state, 0, -1) {
                    game_state.current_player_y -= 1;
                    game_state.cursor_blink = true;
                }

            } else if keycode == KeyCode::Down || keycode == KeyCode::Char('s') {
                // Handle moving the player downward
                if self.check_move_available(game_data, game_state, 0, 1) {
                    game_state.current_player_y += 1;
                    game_state.cursor_blink = true;
                }

            } else if keycode == KeyCode::Left || keycode == KeyCode::Char('a') {
                // Handle moving the player leftward
                if self.check_move_available(game_data, game_state, -1, 0) {
                    game_state.current_player_x -= 1;
                    game_state.cursor_blink = true;
                }

            } else if keycode == KeyCode::Right || keycode == KeyCode::Char('d') {
                // Handle moving the player rightward
                if self.check_move_available(game_data, game_state, 1, 0) {
                    game_state.current_player_x += 1;
                    game_state.cursor_blink = true;
                }

            } else if keycode == KeyCode::Enter {
                // Handle interacting with an object the player is over
                self.handle_interact_key(game_state, game_data);
            } else if keycode == KeyCode::Char('E') || keycode == KeyCode::Char('e') {
                // Handle opening the player's inventory
                game_state.visual_state = VisualState::PlayingInventory;
                execute!(
                        stdout(),
                        MoveTo(10, 11),
                        Print("Changing to Inventory")
                    )?;
            }

            game_state.last_character_processed = true;
            match self.draw(game_data, game_state) {
                Ok(_) => {},
                Err(_) => {
                    println!("ERROR: Problem encountered while drawing screen, exiting!");
                    self.end()?;
                }
            }
        }

        Ok(())
    }


    // Returns true if there is space on the map for the player to move delta_x spaces right and delta_y spaces down.
    // Returns false if there is an object in the way or the move is out of map bounds.
    fn check_move_available(&self, game_data: &GameData, game_state: &mut GameState, delta_x: i16, delta_y: i16) -> bool {

        let target_x = game_state.current_player_x as i16 + delta_x;
        let target_y = game_state.current_player_y as i16 + delta_y;
        let map = &game_data.maps[game_state.current_map];

        // Check to make sure they can't leave the room boundaries (except for doors)
        if target_x > 0 && target_x < (map.grid.len()-1) as i16 &&
           target_y > 0 && target_y < (map.grid[0].len()-1) as i16 {
            // Ok, they're within the room's boundaries, check objects/characters/empty spaces
            if map.grid[target_x as usize][target_y as usize].is_some() {
                // There's something here, find out what it is
                match map.grid[target_x as usize][target_y as usize].as_ref().unwrap() {
                    MapData::Character(_) => {
                        // All characters can be walked over (for interacting)
                        return true;
                    }
                    MapData::Object(object) => {
                        // Only collidable objects can't be walked over
                        match object.category.as_str() {
                            "collidable" => { return false; }
                            _ => { return true; }
                        }
                    }

                }

            } else {
                // Empty space, they can obviously go there
                return true;
            }

        } else if target_x == 0 || target_x == (map.grid.len()-1) as i16 ||
            target_y == 0 || target_y == (map.grid[0].len()-1) as i16 {
            // This is a wall, make sure a door is there
            if map.grid[target_x as usize][target_y as usize].is_some() {
                // There's something here, find out what it is
                match map.grid[target_x as usize][target_y as usize].as_ref().unwrap() {
                    MapData::Object(object) => {
                        match object.category.as_str() {
                            "door" => { return true; }
                            _ => { return false; }
                        }
                    }
                    _ => { return false; } // Although this should never happen (no characters in walls!)
                }
            } else {
                // This is a wall, they cannot enter it
                return false;
            }
        } else {
            // No idea where they are, but they def shouldn't be there and can't go anywhere
            return false;
        }

    }



    // Draws a 50 x 12 dialog box with two buttons. Dialog and button text is specified in game_state.
    // Handles key presses and button highlighting.
    fn draw_playing_dialog(&self, game_data: &mut GameData, game_state: &mut GameState) -> Result<()> {

        // Dialog box width
        let cols = 80;
        let rows = 20;
        let width = 50;
        let height = 12;
        let button_height = 3;

        // Draw dialog box border
        self.draw_border((cols-width)/2, (rows-height)/2,width,height)?;
        // Draw buttons border
        self.draw_border((cols-width)/2, (rows-height)/2+height-button_height, width, button_height)?;
        // Split into 2 buttons
        self.draw_border((cols-width)/2, (rows-height)/2+height-button_height, width/2, button_height)?;

        // Create a wrapped version of the dialog message
        let message = textwrap::wrap(&game_state.dialog_message, (width-4) as usize);

        // Draw the dialog message
        let vertical_start = self.vertically_center_start_position(message.len() as u16, self.current_rows);
        for i in 0..message.len() {
            execute!(
                stdout(),
                //MoveTo((cols-width)/2+2, (rows-height)/2+1+i as u16),
                MoveTo(self.horizontally_center_start_position(&message[i], self.current_columns), vertical_start + i as u16 - 1),
                Print(&message[i]),
            )?;
        }

        // Display selected button highlighting
        if game_state.dialog_selected == 0 {
            for i in 0..width/2-2 {
                execute!(
                    stdout(),
                    MoveTo((cols-width)/2+1+i, (rows-height)/2+height-button_height+1),
                    Print("="),
                )?;
            }
        } else {
            for i in 0..width/2-1 {
                execute!(
                    stdout(),
                    MoveTo((cols-width)/2 + width/2 + i, (rows-height)/2+height-button_height+1),
                    Print("="),
                )?;
            }
        }

        // Display button text
        let center_0 = (cols-width)/2 +
                            self.horizontally_center_start_position(&game_state.dialog_option_0, width/2);
        let center_1 = cols/2 +
                            self.horizontally_center_start_position(&game_state.dialog_option_1, width/2);
        execute!(
            stdout(),
            MoveTo(center_0, (rows-height)/2+height-button_height+1),
            Print(&game_state.dialog_option_0),
            MoveTo(center_1, (rows-height)/2+height-button_height+1),
            Print(&game_state.dialog_option_1),
        )?;

        if !game_state.last_character_processed {

            let keycode = match game_state.last_character_pressed.as_ref().unwrap() {
                Event::Key(x) => {
                    x.code
                },
                _ => { KeyCode::Null }
            };

            // Process exiting the game
            // TODO: Add processing for other key presses here
            // For example, control+C via something like
            // game_state.last_character_pressed.as_ref().unwrap().modifiers == KeyModifiers::CONTROL && keycode == KeyCode::Char('C')
            if keycode == KeyCode::Left {
                if game_state.dialog_selected == 1 {
                    game_state.dialog_selected = 0
                }

            } else if keycode == KeyCode::Right {
                if game_state.dialog_selected == 0 {
                    game_state.dialog_selected = 1
                }

            } else if keycode == KeyCode::Enter {
                // Check if we need to do a full exit or a return to the previous screen
                if game_state.pre_exit {
                    if game_state.dialog_selected == 1 {
                        game_state.do_exit = true;
                    } else { // If dialog_selected == 0
                        game_state.pre_exit = false;
                        game_state.visual_state = game_state.dialog_return_0.clone();
                    }
                } else {
                    // Set up the result and return to the previous screen
                    game_state.dialog_result_ready = true;
                    if game_state.dialog_selected == 0 {
                        game_state.visual_state = game_state.dialog_return_0.clone();
                    } else { // if dialog_selected == 1
                        game_state.visual_state = game_state.dialog_return_1.clone();
                    }
                }

                // Reset selected dialog button
                game_state.dialog_selected = 0;

            } else if keycode == KeyCode::Esc {
                // Set up the (lack of) result and return to the previous screen
                game_state.dialog_result_ready = false;
                game_state.visual_state = game_state.dialog_return_cancel.clone();
                
                // Reset selected dialog button
                game_state.dialog_selected = 0;

            } else if keycode == KeyCode::Char('H') {

                // Change to map view
                game_state.visual_state = VisualState::StartScreen;
                execute!(
                    stdout(),
                    MoveTo(10, 11),
                    Print("Changing to start screen")
                )?;

            } else if keycode == KeyCode::Char('M') {

                // Change to map view
                game_state.visual_state = VisualState::PlayingMap;
                execute!(
                        stdout(),
                        MoveTo(10, 11),
                        Print("Changing to map view")
                    )?;

            }

            game_state.last_character_processed = true;
            match self.draw(game_data, game_state) {
                Ok(_) => {},
                Err(_) => {
                    println!("ERROR: Problem encountered while drawing screen, exiting!");
                    self.end()?;
                }
            }
        }

        Ok(())
    }

    // Draws inventory/stat screen.
    // Handles key presses.
    fn draw_playing_inventory(&self, game_data: &mut GameData, game_state: &mut GameState) -> Result<()> {
        // Inventory screen width
        let cols = self.current_columns;
        let rows = self.current_rows;
        let grid_start_col: u16 = 28;

        // Draw screen border
        self.draw_border(0, 0, cols, rows)?;

        // Draw player face
        self.draw_face(5, 2, game_data.info.player.as_ref().unwrap())?;

        // Draw player stats
        let player_stats = &game_data.info.player.as_ref().unwrap().attributes;
        self.draw_stat_display(player_stats, 2, (rows*2)/3)?;

        // Draw all the items in the inventory
        let inventory = &game_data.info.player.as_ref().unwrap().inventory;
        self.draw_item_grid(inventory, grid_start_col, 0, game_state.inventory_x, game_state.inventory_y)?;

        let inventory_width = inventory.len();
        let inventory_height;
        if inventory_width > 0 {
            inventory_height = inventory[0].len();
        } else {
            inventory_height = 0;
        }

        // If the keypress has not been processed yet, process it.
        if !game_state.last_character_processed {

            // Get keypress
            let keycode = match game_state.last_character_pressed.as_ref().unwrap() {
                Event::Key(x) => {
                    x.code
                },
                _ => { KeyCode::Null }
            };

            // Process keypresses for selecting items
            if keycode == KeyCode::Left {
                if game_state.inventory_x == 0 {
                    game_state.inventory_x = inventory_width - 1
                } else {
                    game_state.inventory_x = game_state.inventory_x - 1;
                }

            } else if keycode == KeyCode::Right {
                game_state.inventory_x = (game_state.inventory_x + 1) % inventory_width;

            } else if keycode == KeyCode::Up {
                if game_state.inventory_y == 0 {
                    game_state.inventory_y = inventory_height - 1
                } else {
                    game_state.inventory_y = game_state.inventory_y - 1;
                }

            } else if keycode == KeyCode::Down {
                game_state.inventory_y = (game_state.inventory_y + 1) % inventory_height;

            } else if keycode == KeyCode::Enter {
                self.use_object(game_state, game_data);
            }

            // Process keypresses for changing screens
            if keycode == KeyCode::Char('H') {
                // Change to start screen
                game_state.visual_state = VisualState::StartScreen;
                execute!(
                    stdout(),
                    MoveTo(10, 11),
                    Print("Changing to start screen")
                )?;
            } else if keycode == KeyCode::Char('m')
                   || keycode == KeyCode::Esc
                   || keycode == KeyCode::Char('e') {
                // Change to map view
                game_state.visual_state = VisualState::PlayingMap;
                execute!(
                        stdout(),
                        MoveTo(10, 11),
                        Print("Changing to map view")
                    )?;
            }

            game_state.last_character_processed = true;
            match self.draw(game_data, game_state) {
                Ok(_) => {},
                Err(_) => {
                    println!("ERROR: Problem encountered while drawing screen, exiting!");
                    self.end()?;
                }
            }
        }

        Ok(())
    }

    // Draws the player interaction screen. Shows the player character and the
    //    NPC being interacted with. Shows the NPC dialog and gives the player
    //    dialog options that can be selected.
    fn draw_playing_character_interaction(&self, game_data: &mut GameData, game_state: &mut GameState) -> Result<()> {
        // Interaction screen width
        let cols = self.current_columns;
        let rows = self.current_rows;
        let dialog_height = 8;

        // Get dialog from hashmap
        let dialog = game_data.dialogs.get(&game_state.npc_dialog_id).unwrap();
        let npc_dialog = &dialog.npc_dialog;
        let dialog_0 = &dialog.option_0.dialog;
        let dialog_1 = &dialog.option_1.dialog;

        // Draw screen borders
        self.draw_border(0, 0, cols, rows)?;
        self.draw_border(cols/2, 0, cols/2, rows)?;

        // Draw dialog options
        self.draw_text_box(0, rows-dialog_height,
             cols/4, dialog_height, &dialog_0, game_state.dialog_selected == 0)?;
        self.draw_text_box((cols/4) - 1, rows-dialog_height,
             (cols/4) + 2, dialog_height, &dialog_1, game_state.dialog_selected == 1)?;

        // Draw NPC dialog
        self.draw_text_box(cols/2, rows-dialog_height,
             cols/2, dialog_height, &npc_dialog, false)?;
        // (cover up the dialog box line)
        stdout().execute(MoveTo((cols/2)+1, rows-dialog_height))?;
        for _i in 0..((cols/2) - 2) {
            stdout().execute(Print(" "))?;
        }
        stdout().execute(Print("|"))?;

        // Draw player face
        self.draw_face(11, 2, game_data.info.player.as_ref().unwrap())?;

        // Get NPC character from this spot on the map and draw their face
        let map = &game_data.maps[game_state.current_map].clone();
        match map.grid[game_state.current_player_x as usize]
                      [game_state.current_player_y as usize].as_ref().unwrap()  {
            MapData::Character(character) => {
                self.draw_face((cols/2) + 11, 2, character)?;
            }
            MapData::Object(_object) => {}
        }

        // If the keypress has not been processed yet, process it.
        if !game_state.last_character_processed {

            // Get keypress
            let keycode = match game_state.last_character_pressed.as_ref().unwrap() {
                Event::Key(x) => {
                    x.code
                },
                _ => { KeyCode::Null }
            };

            // Process keypresses for selecting items
            if keycode == KeyCode::Left {
                game_state.dialog_selected = 0;

            } else if keycode == KeyCode::Right {
                game_state.dialog_selected = 1;

            } else if keycode == KeyCode::Enter {
                let next: &String;
                if game_state.dialog_selected == 0 {
                    next = &dialog.option_0.next;
                } else {
                    next = &dialog.option_1.next;
                }
                // Reset selected dialog
                game_state.dialog_selected = 0;

                if next == "exit" {
                    game_state.visual_state = game_state.dialog_return_cancel.clone();

                } else if next == "inventory" {
                    game_state.visual_state = VisualState::PlayingInventory;

                } else if game_data.dialogs.contains_key(next) { // if next is a dialog id
                    game_state.npc_dialog_id = next.clone();

                }
            }

            // Process keypresses for changing screens
            if keycode == KeyCode::Char('H') {
                // Change to start screen
                game_state.visual_state = VisualState::StartScreen;
                execute!(
                    stdout(),
                    MoveTo(10, 11),
                    Print("Changing to start screen")
                )?;
            } else if keycode == KeyCode::Char('m')
                   || keycode == KeyCode::Esc {
                // Reset selected dialog
                game_state.dialog_selected = 0;
                // Change to map view
                game_state.visual_state = VisualState::PlayingMap;
                execute!(
                        stdout(),
                        MoveTo(10, 11),
                        Print("Changing to map view")
                    )?;
            }

            game_state.last_character_processed = true;
            match self.draw(game_data, game_state) {
                Ok(_) => {},
                Err(_) => {
                    println!("ERROR: Problem encountered while drawing screen, exiting!");
                    self.end()?;
                }
            }
        }

        Ok(())
    }

    // TODO: Implementation, documentation
    fn draw_playing_character_fight(&self, game_data: &GameData, game_state: &mut GameState) -> Result<()> {

        Ok(())
    }

    // Draws the screen specified by VisualState.
    pub fn draw(&self, game_data: &mut GameData, game_state: &mut GameState) -> Result<()> {

        //self.draw_border(0, 0, 80, 20)?;

        match game_state.visual_state {
            VisualState::StartScreen => {
                self.draw_start_screen(game_data, game_state)?;
            },
            VisualState::PlayingMap => {
                self.draw_playing_map(game_data, game_state)?;
            },
            VisualState::PlayingDialog => {
                self.draw_playing_dialog(game_data, game_state)?;
            },
            VisualState::PlayingInventory => {
                self.draw_playing_inventory(game_data, game_state)?;
            },
            VisualState::PlayingCharacterInteraction => {
                self.draw_playing_character_interaction(game_data, game_state)?;
            },
            VisualState::PlayingCharacterFight => {
                self.draw_playing_character_fight(game_data, game_state)?;
            },
        }

        Ok(())
    }


    // Cleans up terminal and displays shutdown message
    pub fn end(&self) -> Result<()> {

        self.reset()?;
        println!("Shutting down, goodbye!");

        Ok(())
    }

    // Resets the terminal to a usable state
    fn reset(&self) -> Result<()> {
        // Be a good citizen and cleanup the terminal for program exit

        // Get rid of raw mode
        disable_raw_mode()?;

        // Turn on the cursor
        stdout().execute(Show)?;

        // Restore the original size
        execute!(stdout(), SetSize(self.original_columns, self.original_rows))?;

        // Clear the screen
        execute!(stdout(), Clear(ClearType::All))?;

        // Move the cursor back down so the user can see the prompt
        execute!(stdout(), MoveTo(0, 0))?;

        Ok(())
    }
}
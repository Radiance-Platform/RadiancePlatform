use std::fmt::Debug;
use crate::game::config_parsers::GameData;
use crate::game::GameState;

use std::io::{Error, ErrorKind, stdout};
use std::process::{exit};

use crossterm::{
    execute,
    style::{Print},
    ExecutableCommand, Result,
    terminal::{ScrollUp, SetSize, size},
    cursor::{MoveTo}
};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
//use crate::game::maps::MapData{Character, Object};
use crate::game::maps::MapData;
use super::objects::{ObjectInteraction, Object};
//use crate::game::objects::Object;

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

    fn check_screen_size() -> Result<()> {
        let error_terminal_too_small = Error::new(ErrorKind::Other, "Terminal size is too small, must be at least 80x20");
        let (cols, rows) = size()?;
        return if cols < 80 || rows < 20 {
            Err(error_terminal_too_small)
        } else {
            Ok(())
        }
    }

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
        stdout().execute(Hide);

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
    fn horizontally_center_start_position(&self, s: &str) -> u16 {
        let empty_space = self.current_columns - (s.len() as u16);
        return empty_space/2;
    }

    // Function to take in a number of lines and determine what row (line) they should start
    // being printed at in order to vertically center them.
    fn vertically_center_start_position(&self, c: u16) -> u16 {
        let empty_space = self.current_rows - c;
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
        game_state.dialog_return_to = game_state.visual_state.clone();
        game_state.visual_state = VisualState::PlayingDialog;
        game_state.pre_exit = true;
    }

    fn handle_interact_key(&self, game_state: &mut GameState, game_data: &GameData) {
        let map = &game_data.maps[game_state.current_map];
        let player_x = game_state.current_player_x as usize;
        let player_y = game_state.current_player_y as usize;
        if map.grid[player_x][player_y].is_some() {
            match map.grid[player_x][player_y].as_ref().unwrap()  {
                MapData::Character(character) => {
                    // TODO: Handle character interaction
                }
                MapData::Object(object) => {
                    self.activate_object(game_state, game_data, object);
                }
            }
        }
    }

    // Starts any interaction that happens when an object is activated with the interact key
    fn activate_object(&self, game_state: &mut GameState, game_data: &GameData, object: &Object) {
        match object.category.as_str() {
            "collectable" => {
                //collect_object(object);
                return;
            }
            _ => {
            }
        }
        for interaction in &object.interactions {
            match interaction {
                ObjectInteraction::ObjectInteractionActivate(activate) => {
                    if !object.prereqs_met(&activate.prereqs) {
                        // TODO: Display a message here?
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

    // Moves character to a different map through the specified door
    // Assume prereqs are already checked.
    fn travel_through_door(&self, game_state: &mut GameState, game_data: &GameData, door: &Object) {
        // find door
        //let current_map = &game_data.maps[game_state.current_map];
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
                                if object.id == door.id { // If correct door is found
                                    // Move character to new door
                                    game_state.current_map = m;
                                    game_state.current_player_x = c as u16;
                                    game_state.current_player_y = r as u16;
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

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

    // TODO: Implementation, documentation
    fn draw_start_screen(&self, game_data: &GameData, game_state: &mut GameState) -> Result<()> {
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
        lines.push("Use space to interact with objects in the world.");
        lines.push("Use E to open your inventory, WASD/Arrows");
        lines.push("to move within, and Enter to select items.");
        lines.push("Press Enter to select options within dialog boxes.");
        lines.push("");
        lines.push("");
        lines.push("Press Enter to start the game.");

        let mut row = self.vertically_center_start_position(lines.len() as u16);
        for line in lines {
            execute!(
                stdout(),
                MoveTo(self.horizontally_center_start_position(line), row),
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

    // TODO: Implementation, documentation
    fn draw_playing_map(&self, game_data: &GameData, game_state: &mut GameState) -> Result<()> {

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

            } else if keycode == KeyCode::Char(' ') {
                // Handle interacting with an object the player is over
                // TODO: Handle player interactions
                self.handle_interact_key(game_state, game_data);
            } else if keycode == KeyCode::Char('E') {
                // Handle opening the player's inventory
                // TODO: Handle opening the player's inventory

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



    // TODO: Implementation, documentation
    fn draw_playing_dialog(&self, game_data: &GameData, game_state: &mut GameState) -> Result<()> {

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
        for i in 0..message.len() {
            execute!(
                stdout(),
                MoveTo((cols-width)/2+2, (rows-height)/2+1+i as u16),
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
        // TODO: Proper centering of the text
        execute!(
            stdout(),
            MoveTo(27, (rows-height)/2+height-button_height+1),
            Print(&game_state.dialog_option_0),
            MoveTo(49, (rows-height)/2+height-button_height+1),
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
                    } else {
                        game_state.pre_exit = false;
                        game_state.visual_state = game_state.dialog_return_to.clone();
                    }
                } else {
                    // Set up the result and return to the previous screen
                    game_state.dialog_result_ready = true;
                }

                // Reset selected dialog button
                game_state.dialog_selected = 0;

            } else if keycode == KeyCode::Esc {
                // Set up the (lack of) result and return to the previous screen
                game_state.dialog_result_ready = false;
                game_state.visual_state = game_state.dialog_return_to.clone();
                
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

    // TODO: Implementation, documentation
    fn draw_playing_inventory(&self, game_data: &GameData, game_state: &mut GameState) -> Result<()> {

        Ok(())
    }

    // TODO: Implementation, documentation
    fn draw_playing_character_interaction(&self, game_data: &GameData, game_state: &mut GameState) -> Result<()> {

        Ok(())
    }

    // TODO: Implementation, documentation
    fn draw_playing_character_fight(&self, game_data: &GameData, game_state: &mut GameState) -> Result<()> {

        Ok(())
    }

    // TODO: Documentation
    pub fn draw(&self, game_data: &GameData, game_state: &mut GameState) -> Result<()> {

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
                self.draw_playing_dialog(game_data, game_state)?;
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


    pub fn end(&self) -> Result<()> {

        self.reset()?;
        println!("Shutting down, goodbye!");

        Ok(())
    }

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
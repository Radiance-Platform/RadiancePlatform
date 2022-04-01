use std::fmt::Debug;
use crate::game::config_parsers::GameData;
use crate::game::GameState;

use std::io::{Error, ErrorKind, stdout, Write};
use std::process::exit;

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand, Result,
    event,
    terminal::{ScrollUp, SetSize, size},
    cursor::{MoveTo}
};
use crossterm::cursor::{DisableBlinking, EnableBlinking, Hide, SetCursorShape, Show};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crate::game::maps::MapData;
use crate::game::maps::MapData::{Character, Object};

pub enum VisualStates {
    StartScreen,
    PlayingMap,
    PlayingDialog,
    PlayingInventory,
    PlayingCharacterInteraction,
    PlayingCharacterFight,
}

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

    // TODO: Function to take in a string and determine what column it should start being printed at
    // in order to horizontally center it. `s` should be the string to center.
    fn horizontally_center_start_position(&self, s: &str) -> u16 {
        // TODO: Read current_columns and length of s to calculate proper positioning
        return 0;
    }

    // TODO: Function to take in a number of lines and determine what row (line) they should start
    // being printed at in order to vertically center them.
    fn vertically_center_start_position(&self, c: u16) -> u16 {
        // TODO: Read current_rows and c to calculate proper positioning
        return 0;
    }

    // Test function to use for making sure I can blink the cursor like when a character is over
    // another object
    fn blink_cursor(&self, game_state: &mut GameState) -> Result<()> {
        stdout().execute(MoveTo(78, 18))?;
        if game_state.cursor_blink {
            stdout().execute(Print("X"))?;
        } else {
            stdout().execute(Print("O"))?;
        }

        game_state.cursor_blink = !game_state.cursor_blink;

        Ok(())
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
        execute!(
                stdout(),
                // TODO: Center these properly
                MoveTo(5, 2),
                Print("Welcome to "),
                Print(&game_data.info.name),
                MoveTo(5, 3),
                Print("Written by "),
                Print(&game_data.info.author),
                MoveTo(5, 4),
                Print("Powered by The Radiance Platform"),
                MoveTo(5, 5),
                Print("WORK IN PROGRESS: Current functionality consists of recognizing"),
                MoveTo(5, 6),
                Print("key presses, and handling pressing `C` to exit."),
            )?;

        if !game_state.last_character_processed {

            let keycode = match game_state.last_character_pressed.as_ref().unwrap() {
                Event::Key(x) => {
                    x.code
                },
                _ => { KeyCode::Null }
            };

            //let keycode2 = game_state.last_character_pressed.as_ref().unwrap();

            // Something to look into
            /* match event {
                    KeyEvent {
                        code: KeyCode::Char('q'),
                        modifiers: event::KeyModifiers::NONE,
                    } => break,
                    _ => {
                    }
                }
             */

            execute!(
                stdout(),
                MoveTo(10, 10),
                Print("Received keycode: "),
                Print(format!("{:?} ", keycode)),
                //Print(format!("{:?}", keycode2)),
            )?;

            // Process exiting the game
            // TODO: Add processing for other key presses here
            // For example, control+C via something like
            // game_state.last_character_pressed.as_ref().unwrap().modifiers == KeyModifiers::CONTROL && keycode == KeyCode::Char('C')
            if keycode == KeyCode::Char('C') {
                execute!(
                    stdout(),
                    MoveTo(10, 11),
                    Print("Would you like to exit? Y/N")
                )?;
                game_state.pre_exit = true;

            } else if game_state.pre_exit {
                if keycode == KeyCode::Char('Y') {
                    game_state.do_exit = true;
                } else {
                    execute!(
                        stdout(),
                        MoveTo(10, 11),
                        Print("Exit aborted")
                    )?;
                    game_state.pre_exit = false;
                }
            } else if keycode == KeyCode::Char('M') {

                // Change to map view
                game_state.visual_state = VisualStates::PlayingMap;
                execute!(
                        stdout(),
                        MoveTo(10, 11),
                        Print("Changing to map view")
                    )?;

            } else if keycode == KeyCode::Char('D') {

                // Change to dialog view
                game_state.visual_state = VisualStates::PlayingDialog;
                execute!(
                        stdout(),
                        MoveTo(10, 11),
                        Print("Changing to dialog view")
                    )?;

            }

            game_state.last_character_processed = true;
        }

        self.blink_cursor(game_state)?;

        Ok(())
    }

    // TODO: Implementation, documentation
    fn draw_playing_map(&self, game_data: &GameData, game_state: &mut GameState) -> Result<()> {

        // Clear the screen
        self.draw_border(0, 0, 80, 20)?;

        // Find the current map that the player is in
        let map = &game_data.maps[game_state.current_map_id];

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
                        Character(character) => {
                            execute!(
                                stdout(),
                                MoveTo(start_c+c as u16, start_r+r as u16),
                                Print(character.icon),
                            )?;
                        }
                        Object(object) => {
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


        if !game_state.last_character_processed {

            let keycode = match game_state.last_character_pressed.as_ref().unwrap() {
                Event::Key(x) => {
                    x.code
                },
                _ => { KeyCode::Null }
            };


            execute!(
                stdout(),
                MoveTo(10, 10),
                Print("Received keycode: "),
                Print(format!("{:?} ", keycode)),
                //Print(format!("{:?}", keycode2)),
            )?;

            // Process exiting the game
            // TODO: Add processing for other key presses here
            // For example, control+C via something like
            // game_state.last_character_pressed.as_ref().unwrap().modifiers == KeyModifiers::CONTROL && keycode == KeyCode::Char('C')
            if keycode == KeyCode::Char('C') {
                game_state.dialog_message = "Would you like to exit the game?".to_string();
                game_state.dialog_option_0 = "No".to_string();
                game_state.dialog_option_1 = "Yes".to_string();
                game_state.visual_state = VisualStates::PlayingDialog;
                game_state.pre_exit = true;

            } else if game_state.pre_exit {
                if keycode == KeyCode::Char('Y') {
                    game_state.do_exit = true;
                } else {
                    game_state.pre_exit = false;
                }
            } else if keycode == KeyCode::Char('M') {

                // Change to the next map
                if game_state.current_map_id + 1 <  game_data.maps.len() {
                    game_state.current_map_id = game_state.current_map_id + 1;
                } else {
                    game_state.current_map_id = 0;
                }

            } else if keycode == KeyCode::Char('H') {

                // Change to home view
                game_state.visual_state = VisualStates::StartScreen;
                execute!(
                        stdout(),
                        MoveTo(10, 11),
                        Print("Changing to start screen")
                    )?;
            }


            game_state.last_character_processed = true;
            match self.draw(game_data, game_state) {
                Ok(_) => {},
                Err(_) => {
                    println!("ERROR: Problem encountered while drawing screen, exiting!");
                    self.end();
                }
            }
        }

        Ok(())
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

        // Display selected button
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
                if game_state.pre_exit {
                    if game_state.dialog_selected == 1 {
                        game_state.do_exit = true;
                    } else {
                        execute!(
                            stdout(),
                            MoveTo(10, 11),
                            Print("Exit aborted")
                        )?;
                        game_state.pre_exit = false;
                    }
                } else {
                    // TODO basically
                }

            } else if keycode == KeyCode::Char('H') {

                // Change to map view
                game_state.visual_state = VisualStates::StartScreen;
                execute!(
                    stdout(),
                    MoveTo(10, 11),
                    Print("Changing to start screen")
                )?;

            } else if keycode == KeyCode::Char('M') {

                // Change to map view
                game_state.visual_state = VisualStates::PlayingMap;
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
                    self.end();
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
            VisualStates::StartScreen => {
                self.draw_start_screen(game_data, game_state)?;
            },
            VisualStates::PlayingMap => {
                self.draw_playing_map(game_data, game_state)?;
            },
            VisualStates::PlayingDialog => {
                self.draw_playing_dialog(game_data, game_state)?;
            },
            VisualStates::PlayingInventory => {
                self.draw_playing_dialog(game_data, game_state)?;
            },
            VisualStates::PlayingCharacterInteraction => {
                self.draw_playing_character_interaction(game_data, game_state)?;
            },
            VisualStates::PlayingCharacterFight => {
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

        Ok(())
    }
}
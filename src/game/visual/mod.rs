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
use crossterm::cursor::CursorShape;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};

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
        let error_terminal_too_small = Error::new(ErrorKind::Other, "Terminal size is too small");
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

    fn draw_border(&self) -> Result<()> {

        // Loop over each row (hard set to 20 for now, TODO: Allow dynamic sizing
        for r in 0..20 {
            // Loop over each column (hard set to 80 for now, TODO: Allow dynamic sizing
            stdout().execute(MoveTo(0, r))?;
            for c in 0..80 {
                if r == 0 || r == 19 {
                    if c == 0 || c == 79 {
                        stdout().execute(Print("+"))?;
                    } else {
                        stdout().execute(Print("-"))?;
                    }
                } else {
                    if c == 0 || c == 79 {
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
            }

            game_state.last_character_processed = true;
        }

        Ok(())
    }

    // TODO: Implementation, documentation
    fn draw_playing_map(&self, game_data: &GameData, game_state: &mut GameState) -> Result<()> {

        Ok(())
    }

    // TODO: Implementation, documentation
    fn draw_playing_dialog(&self, game_data: &GameData, game_state: &mut GameState) -> Result<()> {

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

        self.draw_border()?;

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

    pub fn reset(&self) -> Result<()> {
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
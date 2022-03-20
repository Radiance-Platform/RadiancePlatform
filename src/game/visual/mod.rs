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
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

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
        // Make sure their terminal is big enough
        match Screen::check_screen_size() {
            Err(_) => {
                println!("ERROR: Screen size is too small, please use a larger terminal");
                exit(1);
            },
            Ok(_) => {
                println!("Screen size check passed");
            }
        }

        // Store original and desired sizes in a screen struct
        let (orig_cols, orig_rows) = size().unwrap();
        let screen = Screen {
            original_columns: orig_cols,
            original_rows: orig_rows,
            current_columns: 80,
            current_rows: 20
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
        }

        return screen;

    }


    fn draw_border(&self) -> Result<()> {

        // Loop over each row (hard set to 20 for now, TODO: Allow dynamic sizing)
        for r in 0..20 {
            // Loop over each column (hard set to 80 for now, TODO: Allow dynamic sizing)
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

    pub fn draw(&self, game_data: &GameData, game_state: &mut GameState) -> Result<()> {
        // Resize terminal and scroll up.
/*        execute!(
            stdout(),
            Print(self.current_columns.to_string()),
            Print(", "),
            Print(self.current_rows.to_string()),
            SetSize(self.current_columns, self.current_rows),
            Print("\n"),
            ScrollUp(5)
        )?;

        // using the macro
        execute!(
            stdout(),
            SetForegroundColor(Color::Blue),
            SetBackgroundColor(Color::Red),
            Print("Styled text here.\n"),
            ResetColor
        )?;*/

        self.draw_border()?;

        if !game_state.last_character_processed {

            Event::Key(KeyCode::Esc.into());

/*            let char:String = match game_state.last_character_pressed.as_ref().unwrap() {
                Event::Key(X) => {
                    match X.code {
                        KeyCode::Enter => { "Enter" }
                        KeyCode::Up => { "Up" }
                        KeyCode::Down => { "Down" }
                        KeyCode::Left => { "Left" }
                        KeyCode::Right => { "Right" }
                        KeyCode::Char(c) => { c }
                        _ => { "Unsupported keypress" }
                    }
                },
                _ => { "Non-key event" }
            }.into();*/

            let char2 = match game_state.last_character_pressed.as_ref().unwrap() {
                Event::Key(x) => {
                    x.code
                },
                _ => { KeyCode::Null }
            };

            execute!(
                stdout(),
                MoveTo(10, 10),
                Print("Received character: "),
                Print(format!("{:?}", char2)),
            )?;

            if char2 == KeyCode::Char('C') {
                execute!(
                    stdout(),
                    MoveTo(10, 11),
                    Print("Would you like to exit? Y/N")
                )?;
                game_state.pre_exit = true;

            } else if game_state.pre_exit {
                if char2 == KeyCode::Char('Y') {
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


        // Probably going to need this for the alternating terminal symbols
        // SetCursorShape(pub CursorShape);

        Ok(())
    }


    pub fn end(&self) -> Result<()> {

        self.draw_border()?;
        execute!(
                stdout(),
                MoveTo(10, 10),
                Print("Shutting down, goodbye!"),
                MoveTo(0, 20),
            )?;

        self.reset()?;

        Ok(())
    }

    pub fn reset(&self) -> Result<()> {
        // Be a good citizen and cleanup the terminal for program exit
        disable_raw_mode()?;
        execute!(stdout(), SetSize(self.original_columns, self.original_rows))?;
        Ok(())
    }
}
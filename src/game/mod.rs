use std::path::PathBuf;
use std::process::exit;
use std::ptr::null;
use config_parsers::GameData;
use crate::game::visual::Screen;

use crossterm::event::poll;
use crossterm::{
    cursor::position,
    event::{read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};
use std::time::Duration;


pub mod characters;
pub mod maps;
pub mod objects;
pub mod visual;
pub mod config_parsers;

/// The base Game struct that contains all configuration for the game, but not any of its current state
pub struct Game {
    game_data: GameData,
    game_state: GameState,
    screen: Screen,
}

impl Game {

    /// Create an empty GameState from this Game. This is analogous to beginning a new game
    pub fn initialize(config_path:std::path::PathBuf) -> Game {
        println!("Initializing Game");

        let game_data = config_parsers::process_configs(config_path);

        // Create a display object
        let screen = Screen::initialize();

        let game_state = GameState {
            last_character_pressed: Ok(Event::Key(KeyCode::Enter.into())),
            last_character_processed: true,
            pre_exit: false,
            do_exit: false,
        };

        return Game{game_data, game_state, screen};
    }

    /// Start playing the game
    pub fn start(&mut self) {

        println!("Starting game");
        self.screen.draw(&self.game_data, &mut self.game_state);
        self.run();

    }

    /// Main Game Loop
    pub fn run(&mut self) {

        // Blocking read
        self.game_state.last_character_pressed = read();
        self.game_state.last_character_processed = false;

        self.screen.draw(&self.game_data, &mut self.game_state);

        if self.game_state.do_exit {
            self.end();
        } else {
            self.run();
        }
    }

    pub fn end(&self) {
        self.screen.end();
        exit(0);
    }

}

/// The current state of a Game
pub struct GameState {
    pub last_character_pressed: Result<Event>,
    pub last_character_processed: bool,
    pub pre_exit: bool,
    pub do_exit: bool,
}

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

        // Create a display object
        let screen = Screen::initialize();


        let game_data = config_parsers::process_configs(config_path);

        let game_state = GameState {
            last_character_pressed: Ok(Event::Key(KeyCode::Enter.into())),
            last_character_processed: true
        };

        return Game{game_data, game_state, screen};
    }

    /// Start playing the game
    pub fn start(&mut self) {

        println!("Starting game");
        self.run();

    }

    /// Main Game Loop
    pub fn run(&mut self) {
        self.screen.draw(&self.game_data, &mut self.game_state);

        // Blocking read
        let event = read();

        self.run();
    }

    pub fn end(&self) {
        self.screen.reset();
    }

}

/// The current state of a Game
pub struct GameState {
    pub last_character_pressed: Result<Event>,
    pub last_character_processed: bool
}

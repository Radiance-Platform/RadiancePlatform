use std::process::exit;
use std::time::Duration;
use config_parsers::GameData;
use crate::game::screen::{Screen, VisualStates};

use crossterm::{event::{read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode}, execute, terminal::{disable_raw_mode, enable_raw_mode}, Result, event};
use crate::game::maps::Map;

pub mod characters;
pub mod maps;
pub mod objects;
pub mod screen;
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

        let game_data = config_parsers::GameData::process_configs(config_path);

        println!("{:?}", game_data);

        // Create a display object
        let screen = Screen::initialize();

        let game_state = GameState {
            last_character_pressed: Ok(Event::Key(KeyCode::Enter.into())),
            last_character_processed: true,
            pre_exit: false,
            do_exit: false,
            visual_state: VisualStates::StartScreen,
            cursor_blink: false,
            current_map_id: GameState::map_from_id(&game_data, &game_data.info.starting_map),
            dialog_message: "".to_string(),
            dialog_option_0: "".to_string(),
            dialog_option_1: "".to_string(),
            dialog_selected: 0,
        };

        return Game{game_data, game_state, screen};
    }

    /// Start playing the game
    pub fn start(&mut self) {

        println!("Starting game");
        match self.screen.draw(&self.game_data, &mut self.game_state) {
            Ok(_) => {},
            Err(_) => {
                println!("ERROR: Problem encountered while drawing screen, exiting!");
                self.end();
            }
        }
        self.run();

    }

    /// Main Game Loop
    fn run(&mut self) {

        // Blocking read
        //self.game_state.last_character_pressed = read();

        if crossterm::event::poll(std::time::Duration::from_millis(1000)).expect("Error") {
            self.game_state.last_character_pressed = crossterm::event::read();
            self.game_state.last_character_processed = false;
        }


        match self.screen.draw(&self.game_data, &mut self.game_state) {
            Ok(_) => {},
            Err(_) => {
                println!("ERROR: Problem encountered while drawing screen, exiting!");
                self.end();
            }
        }

        if self.game_state.do_exit {
            self.end();
        } else {
            self.run();
        }
    }

    fn end(&self) {
        let _ = self.screen.end();
        exit(0);
    }

}

/// The current state of a Game
pub struct GameState {
    pub last_character_pressed: Result<Event>,
    pub last_character_processed: bool,
    pub pre_exit: bool,
    pub do_exit: bool,
    pub visual_state: VisualStates,
    pub cursor_blink: bool,
    pub current_map_id: usize,
    pub dialog_message: String,
    pub dialog_option_0: String,
    pub dialog_option_1: String,
    pub dialog_selected: u8,
}

impl GameState {
    pub fn map_from_id(game_data: &GameData, map_id: &String) -> usize {
        // Find the map that has an ID matching the provided map_id string
        for i in 0..game_data.maps.len() {
            if game_data.maps[i].info.id == map_id.to_string() {
                return i;
            }
        }
        panic!("Failed to locate map by ID");

    }
}

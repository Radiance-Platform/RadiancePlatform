use std::process::exit;
use std::time::Duration;
use config_parsers::GameData;
use crate::game::screen::{Screen, VisualState};

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

        // Create a display object
        let screen = Screen::initialize();

        let game_state = GameState {
            last_character_pressed: Ok(Event::Key(KeyCode::Enter.into())),
            last_character_processed: true,
            pre_exit: false,
            do_exit: false,
            visual_state: VisualState::StartScreen,
            cursor_blink: true,
            current_map: GameState::map_from_id(&game_data, &game_data.info.starting_map),
            current_player_x: game_data.info.starting_position_x,
            current_player_y: game_data.info.starting_position_x,
            dialog_message: "".to_string(),
            dialog_option_0: "".to_string(),
            dialog_option_1: "".to_string(),
            dialog_selected: 0,
            dialog_result_ready: false,
            dialog_return_to: VisualState::StartScreen,
        };

        return Game{game_data, game_state, screen};
    }

    /// Start playing the game by drawing the first screen, then running the main game loop
    pub fn start(&mut self) {

        println!("Starting game");
        match self.screen.draw(&mut self.game_data, &mut self.game_state) {
            Ok(_) => {},
            Err(_) => {
                println!("ERROR: Problem encountered while drawing screen, exiting!");
                self.end();
            }
        }
        self.run();

    }

    /// Main Game Loop
    /// This gets called recursively to keep the game time moving forward
    /// This function primarily handles reading in user input, handling screen refreshes, and
    /// shutting the game down when requested.
    fn run(&mut self) {

        // Read input, but timeout after 500ms if no input is received.
        // This timeout allows the game to continue processing things like screen updates without
        // needing to rely on user input.
        if crossterm::event::poll(std::time::Duration::from_millis(500)).expect("Error") {
            self.game_state.last_character_pressed = crossterm::event::read();
            self.game_state.last_character_processed = false;
        }

        // Redraw the screen
        match self.screen.draw(&mut self.game_data, &mut self.game_state) {
            Ok(_) => {},
            Err(_) => {
                println!("ERROR: Problem encountered while drawing screen, exiting!");
                self.end();
            }
        }

        // Check if an exit was requested
        if self.game_state.do_exit {
            self.end();
        } else {
            self.run();
        }
    }

    // Shut down the screen properly, and exit the program
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
    pub visual_state: VisualState,
    pub cursor_blink: bool,
    pub current_map: usize,
    pub current_player_x: u16,
    pub current_player_y: u16,
    pub dialog_message: String,
    pub dialog_option_0: String,
    pub dialog_option_1: String,
    pub dialog_selected: u8,
    pub dialog_result_ready: bool,
    pub dialog_return_to: VisualState
}

impl GameState {
    // Converts a map ID into an actual map object index
    // TODO: Move to maps module
    pub fn map_from_id(game_data: &GameData, map_id: &String) -> usize {
        // Find the map that has an ID matching the provided map_id string by searching all maps
        for i in 0..game_data.maps.len() {
            if game_data.maps[i].info.id == map_id.to_string() {
                return i;
            }
        }
        panic!("Failed to locate map by ID");
    }
}

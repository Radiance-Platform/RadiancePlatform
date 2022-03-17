use std::path::PathBuf;
use config_parsers::GameData;

pub mod characters;
pub mod maps;
pub mod objects;
pub mod visual;
pub mod config_parsers;

/// The base Game struct that contains all configuration for the game, but not any of its current state
pub struct Game {
    game_data: GameData,
    game_state: GameState,
}

impl Game {

    /// Create an empty GameState from this Game. This is analogous to beginning a new game
    pub fn initialize(config_path:std::path::PathBuf) -> Game {

        println!("Initializing Game");

        let game_data = config_parsers::process_configs(config_path);

        let game_state = GameState {
        };

        return Game{ game_data, game_state };
    }

    /// Start playing the game
    pub fn start(&self) {

        println!("Starting game");

    }

}

/// The current state of a Game
pub struct GameState {

}

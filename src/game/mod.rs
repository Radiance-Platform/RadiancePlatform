
/// The base Game struct that contains all configuration for the game, but not any of its current state
pub struct Game {

}

impl Game {

    /// Create an empty GameState from this Game. This is analagous to beginning a new game
    pub fn initialize(self) -> GameState {
        // TODO
        GameState {
            game: self
        }
    }

    /// Load pre-existing save data into a GameState with this Game. Unpopulated fields will use default values
    pub fn load(self, save_data: &[u8]) -> GameState {
        unimplemented!()
    }
}

/// The current state of a Game
pub struct GameState {
    game: Game
}

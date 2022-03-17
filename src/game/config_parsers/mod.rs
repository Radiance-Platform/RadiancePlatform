mod characters;
mod game;
mod maps;
mod objects;

pub struct GameData {
    maps: u8,
}

fn scan_config() {

}

pub fn process_configs(config_path:std::path::PathBuf) -> GameData {
    println!("Parsing configs");
    GameData {
        maps: 1,
    }
}
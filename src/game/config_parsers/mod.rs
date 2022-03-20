use std::path::{Component, Path};
use walkdir::WalkDir;

mod characters;
mod game;
mod maps;
mod objects;

pub struct GameData {
    maps: u8,
}

impl GameData {
    fn scan_config(&mut self, config_path: std::path::PathBuf) {
        // Loop over every file in the provided folder
        for entry in WalkDir::new(config_path) {
            let entry = entry.unwrap();
            println!("{}", entry.path().display());
            let components = entry.path().components().collect::<Vec<_>>();
            println!("{:?}", components);
            /*        match components[1] {
            _ => { println!("{}", _)}
        }*/
        }
    }

}


pub fn process_configs(config_path: std::path::PathBuf) -> GameData {
    println!("Parsing configs");

    let mut game_data = GameData { maps: 0 };

    game_data.scan_config(config_path);

    return game_data;
}
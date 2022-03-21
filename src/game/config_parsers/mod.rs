use std::ffi::OsStr;
use crate::game::maps::Map;
use std::path::{Component, Path};
use std::path::Component::Normal;
use walkdir::WalkDir;
use crate::game::characters::Character;
use crate::game::objects::Object;

mod characters;
mod game;
mod maps;
mod objects;

pub struct GameInfo {
    name: String,
    description: String,
    author: String,
    min_screen_cols: u16,
    min_screen_rows: u16,
}

pub struct GameData {
    maps: Vec<Map>,
    characters: Vec<Character>,
    objects: Vec<Object>,
    info: GameInfo,
}

impl GameData {
    fn scan_config(&mut self, config_path: std::path::PathBuf) {

        // Loop over every file in the provided folder
        for entry in WalkDir::new(config_path).into_iter() // Iterator used to walk directory
            .map(|a| a.expect("Unable to read entry"))// Result -> inner type, panic if failure
            .filter(|a| a.path() // Make sure its a path
                .extension() // with an extension
                .map(|ext| ext == OsStr::new("yaml")) // and that the extension is yaml
                .unwrap_or(false )// include this path in the processed paths if it passed, else don't
            ) {

            println!("{:?}", entry);
            println!("{:?}", entry.path().file_name());

            if entry.path().file_name() == Some(OsStr::new("game.yaml")) {
                game::process_config(self, entry.path());

            } else {
                let parent_opt = entry.path() // Option<> representing the parent's path, starting with current path
                    .parent() // Make sure it has a parent
                    .and_then(|a| a.file_name()) // and that parent has a filename, then convert the option to the Option over the filename
                    .and_then(|a| a.to_str()); // If there is a file name AND that filename can be converted to a Rust &str, convert the option to an Option over the &str

                // if that parent option contained a value, assign that value to the variable `parent`
                if let Some(parent) = parent_opt {
                    // Then check it against our valid parents
                    match parent {
                        "maps" => { maps::process_config(self, entry.path()); }
                        "characters" => { characters::process_config(self, entry.path()); }
                        "objects" => { objects::process_config(self, entry.path()); }
                        _ => { println!("Found unknown file '{:?}', ignoring", entry.path())}
                    }
                }
            }
        }
    }

}


pub fn process_configs(config_path: std::path::PathBuf) -> GameData {
    println!("Parsing configs");

    let mut game_data = GameData{
        maps: Vec::new(),
        characters: Vec::new(),
        objects: Vec::new(),
        info: GameInfo{
            name: "".to_string(),
            description: "".to_string(),
            author: "".to_string(),
            min_screen_cols: 0,
            min_screen_rows: 0
        }
    };

    game_data.scan_config(config_path);

    return game_data;
}
use std::ffi::OsStr;
use crate::game::maps::Map;
use std::path::{Component, Path};
use std::path::Component::Normal;
use walkdir::WalkDir;
use crate::game::characters::Character;
use crate::game::objects::Object;
use crate::game::maps::MapData;
use std::collections::HashMap;

use super::maps::MapItemData;

mod characters;
mod game;
mod maps;
mod objects;

pub struct GameInfo {
    pub name: String,
    pub description: String,
    pub author: String,
    pub min_screen_cols: u16,
    pub min_screen_rows: u16,
}

pub struct GameData {
    pub maps: Vec<Map>,
    pub info: GameInfo,
}

impl GameData {
    pub fn process_configs(config_path: std::path::PathBuf) -> GameData {
        println!("Parsing configs");

        let mut game_data = GameData{
            maps: Vec::new(),
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

    fn scan_config(&mut self, config_path: std::path::PathBuf) {

        // As the configs are read, everything is thrown in these vectors, then after all are read, they get put into the actual map objects
        let mut characters = HashMap::<String, Character>::new();
        let mut objects = HashMap::<String, Object>::new();
        let mut map_item_data = Vec::<MapItemData>::new();

        // TODO: Ensure that the maps are read last so that objects/characters can then be stored in the map

        // Loop over every file in the provided folder
        for entry in WalkDir::new(config_path).into_iter() // Iterator used to walk directory
            .map(|a| a.expect("Unable to read entry"))// Result -> inner type, panic if failure
            .filter(|a| a.path() // Make sure its a path
                .extension() // with an extension
                .map(|ext| ext == OsStr::new("yaml")) // and that the extension is yaml
                .unwrap_or(false )// include this path in the processed paths if it passed, else don't
            ) {

            //println!("{:?}", entry);
            //println!("{:?}", entry.path().file_name());

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
                        "maps" => { maps::process_config_serde(self, &mut map_item_data, entry.path()); }
                        "characters" => { characters::process_config_serde(self, &mut characters, entry.path()); }
                        "objects" => { objects::process_config(self, &mut objects, entry.path()); }
                        _ => { println!("Found unknown file '{:?}', ignoring", entry.path())}
                    }
                }
            }
        }
        self.set_map_grid(map_item_data, characters, objects);
    }

    fn set_map_grid(&mut self, map_item_data: Vec<MapItemData>, characters: HashMap<String, Character>,
                                                                objects: HashMap< String, Object> ) {
        for map_item in map_item_data {
            let mut map = Map{
                grid: vec![]
            };
            map.grid.resize(map_item.size.width as usize, vec![] );
            for i in 0..(map_item.size.width) {
                map.grid[i as usize].resize(map_item.size.height as usize, Option::<MapData>::None);
            }

            for map_object in map_item.objects {
                let object_id = map_object.id;
                let pos_x = map_object.position.x;
                let pos_y = map_object.position.y;
                let character = characters.get(&object_id);
                let object = objects.get(&object_id);
                if character.is_some() {
                    map.grid[pos_x][pos_y] = Option::<MapData>::Some(MapData::Character(character.unwrap().to_owned()));
                }
                if object.is_some() {
                    map.grid[pos_x][pos_y] = Option::<MapData>::Some(MapData::Object(object.unwrap().to_owned()));
                }
            }
            self.maps.push(map);
        }
    }

}


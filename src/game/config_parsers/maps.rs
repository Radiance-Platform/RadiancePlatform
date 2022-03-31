use std::error::Error;
use std::fs;
use std::path::Path;
use yaml_rust::{YamlLoader,Yaml};
use yaml_rust::Yaml::Hash;
use serde_yaml::from_str;
use crate::game::config_parsers::GameData;
use crate::game::maps::{Map, MapObject, Size};
use serde::{Serialize,Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapItemData {
    pub id: String,
    pub description: String,
    pub size: Size,
    pub objects: Vec<MapObject>,
}


pub fn process_config_serde(map_item_data: &mut Vec<MapItemData>, config_path: &Path) -> Result<(),serde_yaml::Error>{
    let file_contents = fs::read_to_string(config_path).unwrap();
    let doc = serde_yaml::from_str::<MapItemData>(&file_contents);
    match doc {
        Ok(parsed) => {
            println!("Map Data Parsed\n {:?}", parsed);
            map_item_data.push(parsed);
        }
        Err(err) =>{
            println!("{}",err);
        }
    }
    Ok(())
}
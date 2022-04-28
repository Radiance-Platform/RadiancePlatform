use std::fs;
use std::path::Path;
use crate::game::maps::{Size};
use serde::{Serialize,Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapItemData {
    pub id: String,
    pub description: String,
    pub size: Size,
    pub objects: Vec<MapObject>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapObject {
    pub id: String,
    pub position: Position,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

// Reads the map config file into a temporary structure (MapItemData) using Serde
pub fn process_config_serde(map_item_data: &mut Vec<MapItemData>, config_path: &Path) -> Result<(),serde_yaml::Error>{
    let file_contents = fs::read_to_string(config_path).unwrap();
    let doc = serde_yaml::from_str::<MapItemData>(&file_contents);
    match doc {
        Ok(parsed) => {
            map_item_data.push(parsed);
        }
        Err(err) =>{
            eprintln!("{}",err);
        }
    }
    Ok(())
}
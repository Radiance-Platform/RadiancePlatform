use serde::{Serialize,Deserialize};
use crate::game::characters::Character;
use crate::game::objects::Object;

// Map data structure. Contains a single map with a grid of spaces.
// Each space can hold either nothing, a character, or an object.

#[derive(Clone, Debug)]
pub enum MapData {
    Character(Character),
    Object(Object)
}

#[derive(Debug, Clone)]
pub struct Map {
    pub info: MapInfo,
    pub grid: Vec<Vec<Option<MapData>>>
}

#[derive(Debug, Clone)]
pub struct MapInfo {
    pub id: String,
    pub description: String,
    pub size: Size,
}

#[derive(Debug,Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Size {
    pub width: i64,
    pub height: i64,
}


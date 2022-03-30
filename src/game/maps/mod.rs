use serde::{Serialize,Deserialize};
use crate::game::characters::Character;
use crate::game::objects::Object;

#[derive(Clone)]
pub enum MapData {
    Character(Character),
    Object(Object)
}

pub struct Map {
    pub grid: Vec<Vec<Option<MapData>>>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapItemData {
    pub id: String,
    pub description: String,
    pub size: Size,
    pub objects: Vec<MapObject>,
}

#[derive(Debug,Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Size {
    pub width: i64,
    pub height: i64,
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

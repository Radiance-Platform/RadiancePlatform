use crate::game::characters::Character;
use crate::game::objects::Object;

pub enum MapData {
    Character(Character),
    Object(Object)
}

pub struct Map {
    pub grid: Vec<Vec<Option<MapData>>>
}
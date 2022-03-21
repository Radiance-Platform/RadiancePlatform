

pub enum MapData {
    Character(),
    Object()
}

pub struct Map {
    pub grid: Vec<Vec<Option<MapData>>>
}
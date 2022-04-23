
#[derive(Clone, Debug)]
pub struct Interactions {
    pub attacks: Vec<Attack>,
    pub object_use: Vec<ObjectUse>,
}

#[derive(Clone, Debug)]
pub struct Attack {
    pub id: String,
    pub display_name: String,
    pub base_damage: i64,
    pub affected_by: Vec<Modifier>,
}

#[derive(Clone, Debug)]
pub struct ObjectUse {
    pub object_id: String,
    pub set_dialog: String, // Dialog id. If empty, does not modify dialog
    pub consume_item: bool, // Is the item destroyed upon use?
}

#[derive(Clone, Debug)]
pub struct Modifier {
    pub attribute_id: String,
    pub sign: char,
    pub value: f32,
}
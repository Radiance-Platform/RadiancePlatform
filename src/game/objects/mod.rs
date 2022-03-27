
#[derive(Debug)]
pub struct ObjectState {
    pub name: String,
    pub value: bool
}

pub struct ObjectInteraction {
    pub name: String,
    pub values: String
}

pub struct Object {
    pub id: String,
    pub name: String,
    pub category: String,
    pub icon: char,
    pub state: Vec<ObjectState>,
    pub interactions: Vec<ObjectInteraction>,
}

impl Object {

}
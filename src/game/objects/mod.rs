

pub struct ObjectState {
    pub name: String,
    pub values: String
}

pub struct ObjectInteraction {
    pub name: String,
    pub values: String
}

pub struct Object {
    pub name: String,
    pub icon: char,
    pub state: Vec<ObjectState>,
    pub interactions: Vec<ObjectInteraction>,
}

impl Object {

}
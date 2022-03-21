

pub struct ObjectState {
    pub name: String,
    pub values: Vec<String>
}

pub struct ObjectInteractions {
    pub name: String,
    pub values: Vec<String>
}

pub struct Object {
    pub name: String,
    pub icon: char,
    pub state: Vec<ObjectState>,
    pub interactions: Vec<ObjectInteractions>,
}

impl Object {

}
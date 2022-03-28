
#[derive(Debug)]
pub struct ObjectState {
    pub name: String,
    pub value: bool
}

pub struct ObjectInteraction {
    pub category: ObjectInteractionCategory,
    pub values: String
}

pub enum ObjectInteractionCategory {
    ObjectInteractionActivate(ObjectInteractionActivate),
    ObjectInteractionWorld(ObjectInteractionWorld)
}

pub struct ObjectInteractionActivate {

}

pub struct ObjectInteractionWorld {

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
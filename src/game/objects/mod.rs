use serde::{Serialize,Deserialize};

#[derive(Clone, Debug)]
pub struct ObjectState {
    pub name: String,
    pub value: bool
}

#[derive(Clone, Debug)]
pub struct ObjectInteraction {
    pub category: ObjectInteractionCategory,
    pub values: String
}

#[derive(Clone, Debug)]
pub enum ObjectInteractionCategory {
    ObjectInteractionActivate(ObjectInteractionActivate),
    ObjectInteractionWorld(ObjectInteractionWorld)
}

#[derive(Clone, Debug)]
pub struct ObjectInteractionActivate {

}

#[derive(Clone, Debug)]
pub struct ObjectInteractionWorld {

}

#[derive(Clone, Debug)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectData {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub icon: String,
    pub state: Vec<State>,
    pub interactions: Interactions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub id: String,
    pub default: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Interactions {
    pub activate: Vec<Activate>,
    pub world: Vec<World>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Activate {
    #[serde(rename = "type")]
    pub type_field: String,
    pub prereqs: Prereqs,
    pub destination: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Prereqs {
    pub state: Vec<ActivateState>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivateState {
    pub id: String,
    pub value: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct World {
    #[serde(rename = "type")]
    pub type_field: String,
    pub affected_by: Vec<AffectedBy>,
    pub self_action: SelfAction,
    pub other_action: OtherAction,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AffectedBy {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfAction {
    pub set_state: Vec<SetState>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetState {
    pub unlocked: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherAction {
    pub consume_item: bool,
}

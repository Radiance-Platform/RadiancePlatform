use serde::{Serialize,Deserialize};

#[derive(Clone, Debug)]
pub struct ObjectState {
    pub name: String,
    pub value: bool
}


/*#[derive(Clone, Debug)]
pub struct ObjectInteraction {
    pub category: ObjectInteractionCategory,
    pub values: String
}*/


#[derive(Clone, Debug)]
pub enum ObjectInteraction {
    ObjectInteractionActivate(ObjectInteractionActivate),
    ObjectInteractionObjectUse(ObjectInteractionObjectUse)
}

#[derive(Clone, Debug)]
pub struct ObjectInteractionActivate {
    pub category: String,
    pub prereqs: Vec<ObjectState>,
    pub destination: Option<String>
}

#[derive(Clone, Debug)]
pub struct ObjectInteractionObjectUse{
    pub foreign_object_id: String,
    pub self_action: Vec<ObjectState>,
    pub consume_item: bool
}

#[derive(Clone, Debug)]
pub struct ObjectInteractionWorld {

}

// TODO: Object's categories should probably be some sort of type rather than arbitrary strings
/*pub enum ObjectCategory {
"simple",
"collidable"
"collectable"
"door"
}*/

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
    pub fn prereqs_met(&self, prereqs: &Vec<ObjectState>) -> bool {
        for prereq in prereqs {
            if self.get_state(&prereq.name) != prereq.value {
                return false;
            }
        }
        return true;
    }

    pub fn get_state(&self, state_name: &String) -> bool {
        for s in &self.state {
            if s.name.eq(state_name) {
                return s.value;
            }
        }
        return false;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectData {
    pub id: String,
    pub name: String,
    pub category: String,
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
    pub category: String,
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
    pub category: String,
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

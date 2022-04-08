use serde::{Serialize,Deserialize};
use crate::game::characters::attribute::Attribute;
use crate::game::characters::role::Role;
use crate::game::objects::Object;

pub mod role;
pub mod attribute;


#[derive(Clone, Debug)]
pub struct Character {
    pub name: String,
    pub role: Role,
    pub attributes: Vec<Attribute>,
    pub inventory: Vec<Vec<Option<Object>>>,
    pub icon: char,
}

impl Character {
    pub fn collect_object(&mut self, object: &Object) {
        for c in 0..self.inventory.len() {
            for r in 0..self.inventory[c].len() {
                if self.inventory[c][r].is_none() {
                    self.inventory[c][r] = Option::Some::<Object>(object.to_owned());
                }
            }
        }
    }

}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterData {
    pub id: String,
    pub name: String,
    pub icon: char,
    pub inventory_size: InventorySize,
    pub traits: Vec<Trait>,
    pub interactions: Interactions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventorySize {
    pub width: i64,
    pub height: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trait {
    pub name: String,
    pub display_name: String,
    pub starting_value: u8,
    pub max_value: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Interactions {
    pub attacks: Vec<Attack>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attack {
    pub name: String,
    pub display_name: String,
    pub base_damage: i64,
    pub affected_by: Vec<AffectedBy>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AffectedBy {
    pub skill: String,
    pub effect_per_point: String,
}
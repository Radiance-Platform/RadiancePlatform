
use crate::game::characters::attribute::Attribute;
use crate::game::characters::role::Role;
use crate::game::objects::Object;

pub mod role;
pub mod attribute;


#[derive(Clone, Debug)]
pub struct Character {
    pub id: String,
    pub name: String,
    pub role: Role,
    pub attributes: Vec<Attribute>,
    pub inventory: Vec<Vec<Option<Object>>>,
    pub icon: char,
    pub dialog_id: String,
}

impl Character {
    // Adds specified object to inventory
    pub fn collect_object(&mut self, object: &Object) {
        for c in 0..self.inventory.len() {
            for r in 0..self.inventory[c].len() {
                if self.inventory[c][r].is_none() {
                    self.inventory[c][r] = Option::Some::<Object>(object.to_owned());
                    return;
                }
            }
        }
    }

}


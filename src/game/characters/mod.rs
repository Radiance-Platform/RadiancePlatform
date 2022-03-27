use crate::game::characters::attribute::Attribute;
use crate::game::characters::role::Role;

pub mod role;
pub mod attribute;



pub struct Character {
    pub name: String,
    pub role: Role,
    pub attributes: Vec<Attribute>,
}

impl Character {

}
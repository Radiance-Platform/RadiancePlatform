use crate::game::characters::attribute::Attribute;
use crate::game::characters::role::Role;

mod role;
mod attribute;



pub struct Character {
    name: String,
    role: Role,
    traits: Vec<Attribute>,
}

impl Character {

}
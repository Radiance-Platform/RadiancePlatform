

#[derive(Clone, Debug)]
pub struct Attribute {
    pub name: String,
    pub display_name: String,
    pub min_val: u8,
    pub max_val: u8,
    pub current_val: u8
}
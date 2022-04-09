use std::error::Error;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use yaml_rust::YamlLoader;
use serde::{Serialize,Deserialize};
use crate::game::characters::{Character};
use crate::game::characters::role::Role;
use crate::game::config_parsers::GameData;
use crate::game::characters::attribute::Attribute;
use crate::game::objects::Object;


pub fn process_config_serde(game_data: & mut GameData, 
                            characters: &mut HashMap<String, Character>, config_path: &Path) -> Result<(), serde_yaml::Error>{
    let file_contents = fs::read_to_string(config_path).unwrap();
    let doc = serde_yaml::from_str::<CharacterData>(&file_contents);
    match doc {
        Ok(parsed) =>{
            get_character_from_data( characters, parsed);
        }
        Err(err) =>{
            println!("{}", err);
        }
    }
    Ok(())
}

fn get_character_from_data(characters: &mut HashMap<String, Character>, data: CharacterData) {
    let mut character = Character{
        name: "".to_string(),
        role: Role { role: "".to_string() },
        attributes: vec![],
        inventory: vec![],
        icon: ' ',
    };
    character.inventory.resize(data.inventory_size.width as usize, vec![] );
    for i in 0..(data.inventory_size.width) {
        character.inventory[i as usize].resize(data.inventory_size.height as usize, Option::<Object>::None);
    }
    character.name = data.id;
    character.icon = data.icon;
    //character.role = data.role;
    for attribute_data in data.traits {
        let mut attribute = Attribute {
            name: attribute_data.name,
            min_val: 0,
            max_val: attribute_data.max_value,
            current_val: attribute_data.starting_value
        };
        character.attributes.push(attribute);
    }
    characters.insert(character.name.clone(), character);
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
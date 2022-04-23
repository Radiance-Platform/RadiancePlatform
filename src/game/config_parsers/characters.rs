use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Serialize,Deserialize};
use crate::game::characters::{Character, interactions};
use crate::game::characters::attribute::Attribute;
use crate::game::objects::Object;


// Reads character config file into a temporary data structure using Serde
pub fn process_config_serde(characters: &mut HashMap<String, Character>, config_path: &Path) -> Result<(), serde_yaml::Error>{
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

// Converts temporary data structure in to a Character structure and adds it to the characters list
// so that it can later be added to the game map.
fn get_character_from_data(characters: &mut HashMap<String, Character>, data: CharacterData) {
    let interactions = interactions::Interactions {
        attacks: vec![],
        object_use: vec![],
    };
    let mut character = Character{
        id: "".to_string(),
        name: "".to_string(),
        attributes: vec![],
        inventory: vec![],
        icon: ' ',
        interactions: interactions,
        dialog_id: "".to_string(),
    };
    character.inventory.resize(data.inventory_size.width as usize, vec![] );
    for i in 0..(data.inventory_size.width) {
        character.inventory[i as usize].resize(data.inventory_size.height as usize, Option::<Object>::None);
    }
    character.id = data.id;
    character.name = data.name;
    character.icon = data.icon;
    character.dialog_id = data.dialog_id;
    for attribute_data in data.traits {
        let attribute = Attribute {
            id: attribute_data.id,
            display_name: attribute_data.display_name,
            min_val: 0,
            max_val: attribute_data.max_value,
            current_val: attribute_data.starting_value
        };
        character.attributes.push(attribute);
    }
    for object_use_data in data.interactions.object_use {
        let object_use = interactions::ObjectUse {
            object_id: object_use_data.object_id,
            set_dialog: object_use_data.set_dialog,
            consume_item: object_use_data.consume_item,
        };
        character.interactions.object_use.push(object_use);
    }
    for attack_data in data.interactions.attacks {
        let mut attack = interactions::Attack {
            id: attack_data.id,
            display_name: attack_data.display_name,
            base_damage: attack_data.base_damage,
            affected_by: vec![],
        };
        for modifier_data in attack_data.affected_by {
            let sign = modifier_data.effect_per_point.chars().next().unwrap();
            let value: f32 = modifier_data.effect_per_point[1..].parse().unwrap();
            attack.affected_by.push(interactions::Modifier {
                attribute_id: modifier_data.attribute_id,
                sign: sign,
                value: value,
            });
        }
        character.interactions.attacks.push(attack);
    }
    //println!("CHARACTER: {:?}", character);
    characters.insert(character.id.clone(), character);
}

// Temporary data structure CharacterData is used for Serde parsing and nothing else.

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterData {
    pub id: String,
    pub name: String,
    pub icon: char,
    pub inventory_size: InventorySize,
    pub traits: Vec<Trait>,
    pub interactions: Interactions,
    pub dialog_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventorySize {
    pub width: i64,
    pub height: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trait {
    pub id: String,
    pub display_name: String,
    pub starting_value: u8,
    pub max_value: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Interactions {
    pub attacks: Vec<Attack>,
    pub object_use: Vec<ObjectUseData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attack {
    pub id: String,
    pub display_name: String,
    pub base_damage: i64,
    pub affected_by: Vec<AffectedBy>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AffectedBy {
    pub attribute_id: String,
    pub effect_per_point: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectUseData {
    pub object_id: String,
    pub set_dialog: String,
    pub consume_item: bool,
}
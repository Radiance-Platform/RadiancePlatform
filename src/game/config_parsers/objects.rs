use std::error::Error;
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use yaml_rust::YamlLoader;
use yaml_rust::Yaml;
use crate::game::objects::{Object, ObjectState, ObjectInteraction, ObjectInteractionActivate,
                           ObjectInteractionObjectUse};

// Takes an object config file and loads it into an object, then adds that object to the objects list
// so that it can later be added to the game map.
pub fn process_config(objects: &mut HashMap<String, Object>, config_path: &Path) -> Result<(), Box<dyn Error>> {

    // Load file contents
    let file_contents = fs::read_to_string(config_path)?;

    // Convert to YAML
    let docs = YamlLoader::load_from_str(&*file_contents).unwrap();

    // Multi document support, doc is a yaml::Yaml, need to extract the doc
    let doc = &docs[0];

    let mut object = Object{
        id: "".to_string(),
        name: "".to_string(),
        category: "".to_string(),
        icon: ' ',
        state: vec![],
        interactions: vec![]
    };

    let object_hash = doc.as_hash().unwrap();

    for key in object_hash.keys() {
        let val = object_hash.get(key).unwrap_or(key);
        let key_str = key.as_str().unwrap();
        if val == &Yaml::Null { // sometimes there are no states/interactions specified
            continue;
        }
        match key_str {
            "id" => {
                object.id = val.as_str().unwrap().to_string();
            }
            "name" => {
                object.name = val.as_str().unwrap().to_string();
            }
            "category" => {
                object.category = val.as_str().unwrap().to_string();
            }
            "icon" => {
                object.icon = val.as_str().unwrap().chars().take(1).last().unwrap();
            }
            "state" => {
                parse_object_states(&mut object, val.as_vec().unwrap());
            }
            "interactions" => {
                parse_object_interactions(&mut object, val);
            }
            _ => {continue;}
        }
    }

    objects.insert(object.id.clone(), object);

    Ok(())
}


// Parsing functions for individual components of the object are below.

fn parse_object_interactions( object: &mut Object, yaml_interactions: &Yaml) {
    let interaction_hash = yaml_interactions.as_hash().unwrap();
    for key in interaction_hash.keys() {
        let val = interaction_hash.get(key).unwrap_or(key);
        let key_str = key.as_str().unwrap();
        match key_str {
            "activate" => {
                parse_interaction_activate(object, val);
            }
            "object_use" => {
                parse_interaction_object_use(object, val);
            }
            _ => { continue; }
        }
    }
}

fn parse_interaction_activate( object: &mut Object, yaml_activate: &Yaml) {
    for activation in yaml_activate.as_vec().unwrap() {
        let mut interaction = ObjectInteractionActivate {
            category: "".to_string(),
            prereqs: vec![],
            destination: Option::None
        };
        let activation_hash = activation.as_hash().unwrap();
        for key in activation_hash.keys() {
            let val = activation_hash .get(key).unwrap();
            let key_str = key.as_str().unwrap();
            match key_str {
                "category" => {
                    interaction.category = val.as_str().unwrap().to_string();
                }
                "prereqs" => {
                    parse_state_changes(&mut interaction.prereqs, val);
                }
                "destination" => {
                    interaction.destination = Option::Some(val.as_str().unwrap().to_string());
                }
                _ => { continue; }
            }
        }
        object.interactions.push(ObjectInteraction::ObjectInteractionActivate(interaction));
    }
}

fn parse_interaction_object_use( object: &mut Object, yaml_activate: &Yaml) {
    for activation in yaml_activate.as_vec().unwrap() {
        let mut interaction = ObjectInteractionObjectUse {
            foreign_object_id: "".to_string(),
            self_action: vec![],
            consume_item: false
        };
        let activation_hash = activation.as_hash().unwrap();
        for key in activation_hash.keys() {
            let val = activation_hash .get(key).unwrap();
            let key_str = key.as_str().unwrap();
            match key_str {
                "foreign_objects_id" => {
                    interaction.foreign_object_id = val.as_str().unwrap().to_string();
                }
                "self_action" => {
                    parse_state_changes(&mut interaction.self_action, val);
                }
                "consume_item" => {
                    interaction.consume_item = val.as_bool().unwrap();
                }
                _ => { continue; }
            }
        }
        object.interactions.push(ObjectInteraction::ObjectInteractionObjectUse(interaction));
    }
}

fn parse_state_changes(state_changes: &mut Vec<ObjectState>, yaml_changes: &Yaml) {
    for change in yaml_changes.as_vec().unwrap() {
        let mut state = ObjectState{
            name: "".to_string(),
            value: true
        };
        let state_hash = change.as_hash().unwrap();
        for key in state_hash.keys() {
            let val = state_hash.get(key).unwrap();
            let key_str = key.as_str().unwrap();
            state.name = key_str.to_string();
            state.value = val.as_bool().unwrap();
        }
        state_changes.push(state);
    }
}

fn parse_object_states(object: &mut Object, yaml_states: &[Yaml]) {
    for yaml_state in yaml_states {
        let mut state = ObjectState{
            name: "".to_string(),
            value: true
        };
        let state_hash = yaml_state.as_hash().unwrap();
        for key in state_hash.keys() {
            let val = state_hash
                            .get(key).unwrap();
            let key_str = key.as_str().unwrap();
            match key_str {
                "id" => {
                    state.name = val.as_str().unwrap().to_string();
                }
                "default" => {
                    state.value = val.as_bool().unwrap();
                }
                _ => { continue; }
            }
        }
        object.state.push(state);
    }
}

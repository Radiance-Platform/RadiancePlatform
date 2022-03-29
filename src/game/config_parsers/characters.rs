use std::error::Error;
use std::fs;
use std::path::Path;
use yaml_rust::YamlLoader;
use crate::game::characters::Character;
use crate::game::characters::role::Role;
use crate::game::config_parsers::GameData;

pub fn process_config(game_data: &mut GameData, characters: &mut Vec<Character>, config_path: &Path) -> Result<(), Box<dyn Error>> {

    // Load file contents
    let file_contents = fs::read_to_string(config_path)?;

    // Convert to YAML
    let docs = YamlLoader::load_from_str(&*file_contents).unwrap();
    // Multi document support, doc is a yaml::Yaml, need to extract the doc
    let doc = &docs[0];

    // Debug print
    //println!("{:?}", doc);

    // Todo: Parse fields
    let character = Character{
        name: "".to_string(),
        role: Role { role: "".to_string() },
        attributes: vec![]
    };
    // ...

    characters.push(character);

    Ok(())
}
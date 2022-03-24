use std::error::Error;
use std::fs;
use std::path::Path;
use yaml_rust::YamlLoader;
use crate::game::config_parsers::{GameData, GameInfo};

pub fn process_config(game_data: &mut GameData, config_path: &Path) -> Result<(), Box<dyn Error>> {

    // Load file contents
    let file_contents = fs::read_to_string(config_path)?;

    // Convert to YAML
    let docs = YamlLoader::load_from_str(&*file_contents).unwrap();
    // Multi document support, doc is a yaml::Yaml, need to extract the doc
    let doc = &docs[0];

    // Debug print
    println!("GAME DOC: {:?}\n", doc);

    let game_data_hash = doc.as_hash().unwrap();

    let key = yaml_rust::Yaml::from_str("name");
    let name = game_data_hash.get(&key).unwrap_or(&key).as_str().unwrap();

    println!("GAME NAME: {:?}\n", name);

    // Todo: Parse fields
    game_data.info.name = name.to_string();
    game_data.info.author = "The Radiance Team".to_string();
    // ...

    Ok(())
}

/* TO DO: Add function that takes the name of the yaml field and returns the data
   associated with it completely unpacked as a string */
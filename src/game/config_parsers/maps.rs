use std::error::Error;
use std::fs;
use std::path::Path;
use yaml_rust::{YamlLoader,Yaml};
use yaml_rust::Yaml::Hash;
use crate::game::config_parsers::GameData;
use crate::game::maps::Map;

pub fn process_config(game_data: &mut GameData, config_path: &Path) -> Result<(), Box<dyn Error>> {

    // Load file contents
    let file_contents = fs::read_to_string(config_path)?;

    // Convert to YAML
    let docs = YamlLoader::load_from_str(&*file_contents).unwrap();
    // Multi document support, doc is a yaml::Yaml, need to extract the doc
    let doc:&Yaml = &docs[0];

    // Debug print
    println!("Map YAML:\n{:?}", doc);
    println!("Unwrapped Doc?\n");
    for (k,v) in doc.as_hash().unwrap(){
        println!("{:?} : {:?}",k,v);
        for(kk,vv) in v.as_hash().unwrap_or_else(|| LinkedHashMap){
            println!("{:?} : {:?}",kk,vv);
        }
    }
    // Todo: Parse fields
    let map = Map{ grid: vec![] };
    // ... 

    game_data.maps.push(map);

    Ok(())
}
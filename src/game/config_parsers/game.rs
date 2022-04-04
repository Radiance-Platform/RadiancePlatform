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
    //println!("GAME DOC: {:?}\n", doc);

    let game_info_hash = doc.as_hash().unwrap();

    for key in game_info_hash.keys() {
        let val = game_info_hash.get(&key).unwrap_or(&key);
        let key_str = key.as_str().unwrap();
        match key_str {
            "name" => {
                game_data.info.name = val.as_str().unwrap().to_string();
            }
            "description" => {
                game_data.info.description = val.as_str().unwrap().to_string();
            }
            "author" => {
                game_data.info.author = val.as_str().unwrap().to_string();
            }
            "min_screen_size" => {
                let width_key = yaml_rust::Yaml::from_str("width");
                let width = val.as_hash().unwrap()
                    .get(&width_key).unwrap_or(&key)
                    .as_i64().unwrap();
                let height_key = yaml_rust::Yaml::from_str("height");
                let height = val.as_hash().unwrap()
                    .get(&height_key).unwrap_or(&key)
                    .as_i64().unwrap();

                game_data.info.min_screen_cols = width as u16;
                game_data.info.min_screen_rows = height as u16;
            }
            "starting_map" => {
                game_data.info.starting_map = val.as_str().unwrap().to_string();
            }
            "starting_position" => {
                let x_key = yaml_rust::Yaml::from_str("x");
                let x = val.as_hash().unwrap()
                    .get(&x_key).unwrap_or(&key)
                    .as_i64().unwrap();
                let y_key = yaml_rust::Yaml::from_str("y");
                let y = val.as_hash().unwrap()
                    .get(&y_key).unwrap_or(&key)
                    .as_i64().unwrap();
                game_data.info.starting_position_x = x as u16;
                game_data.info.starting_position_y = y as u16;
            }
            _ => {continue;}
        }
    }

    Ok(())
}
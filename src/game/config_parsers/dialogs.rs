use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::game::dialogs::Dialog;

pub fn process_config_serde(dialogs: &mut HashMap<String, Dialog>, config_path: &Path) -> Result<(), serde_yaml::Error>{
    let file_contents = fs::read_to_string(config_path).unwrap();
    let doc = serde_yaml::from_str::<Vec<Dialog>>(&file_contents);
    match doc {
        Ok(parsed) =>{
            for dialog in parsed {
                dialogs.insert(dialog.id.clone(), dialog.clone());
            }
        }
        Err(err) =>{
            eprintln!("{}", err);
        }
    }
    Ok(())
}
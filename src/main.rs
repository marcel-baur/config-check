use std::collections::HashMap;

use config::Config;
use propparse::{
    fetch_file,
    parser::{Entry, Key},
};
use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Configuration {
    files: Vec<String>,
}

fn get_config() -> Option<Configuration> {
    let settings = match Config::builder()
        .add_source(config::File::with_name("config_check"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
    {
        Ok(s) => s,
        Err(_) => return None,
    };

    match settings.try_deserialize::<Configuration>() {
        Ok(c) => Some(c),
        Err(_) => None,
    }
}
fn main() {
    let configuration = get_config().unwrap();
    let mut key_map = HashMap::<String, Vec<String>>::new();
    // let mut key_vec = Vec::new();
    for file in &configuration.files {
        let result: Vec<Entry> = match fetch_file(&file) {
            Ok(r) => r,
            Err(_) => panic!("Error when parsing file {file}"),
        };
        let keys: Vec<Key> = result.into_iter().map(|r| r.0).collect();
        // key_map.insert(file.clone(), keys.clone());
        // key_vec.push((keys, file));
        for key in keys {
            match key_map.get_mut(&key.join(".")) {
                Some(res) => res.push(file.clone()),
                None => {
                    key_map.insert(key.join(".").clone(), vec![file.clone()]);
                }
            }
        }
    }
    let outliers: Vec<(String, Vec<String>)> = key_map
        .into_iter()
        .filter(|e| e.1.len() < configuration.files.len())
        .collect();
    println!(
        "The following properties only appear in the listed files: {:?}",
        outliers
    );
    for outlier in outliers {
        let variable = outlier.0;
        println!("{}: {}{}","Outiler".bold(), variable.bold().yellow(),  ". This parameter dows not appear in the following files:");
        let not_in: Vec<String> = configuration.files.clone().into_iter().filter(|f| !outlier.1.contains(f)).collect();
        for file in not_in {
            println!("    {}", file.yellow());
        }
    }
}

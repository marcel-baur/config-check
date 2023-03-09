use std::collections::HashMap;

use config::Config;
use serde::{Deserialize, Serialize};
use propparse::{fetch_file, parser::{Entry, Key}};

#[derive(Debug, Serialize, Deserialize)]
struct Configuration {
    files: Vec<String>,
}

fn get_config() -> Option<Configuration> {
    let settings = match Config::builder()
        .add_source(config::File::with_name("config_check"))
        .add_source(config::Environment::with_prefix("APP"))
        .build(){
            Ok(s) => s,
            Err(_) => return None,
        };
        
    match settings.try_deserialize::<Configuration>(){
        Ok(c) => Some(c),
        Err(_) => None
    }
}
fn main() {
    println!("Hello, world!");
    let configuration = get_config().unwrap();    
    let mut key_map = HashMap::new();
    for file in configuration.files {
        let result: Vec<Entry> = match fetch_file(&file) {
            Ok(r) => r,
            Err(_) => panic!("Error when parsing file {file}"),
        };
        let keys: Vec<Key> = result.into_iter().map(|r| r.0).collect();
        key_map.insert(file, keys);
    }
    // TODO: compare entries in map, print differences
}

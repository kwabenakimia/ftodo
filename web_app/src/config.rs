use std::collections::HashMap;
use std::env;

use serde_yaml;

pub struct Config {
  pub map: HashMap<String, serde_yaml::Value>
}

impl Config {
  pub fn new() -> Config {
    let args: Vec<String> = env::args().collect();
    let file_path =&args[args.len()-1];
    println!("Reading config file: {}", file_path);

    let file = std::fs::File::open(file_path).unwrap();
    let map : HashMap<String, serde_yaml::Value> = serde_yaml::from_reader(file).unwrap();

    return Config{map}
  }
}
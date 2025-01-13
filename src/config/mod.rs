use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Config {
    // Enter your fields here needed for configuring the CLI
}

// TODO: improve error handling
pub fn parse_config(path: &str) -> Config {
    let yaml_contents = std::fs::read_to_string(path).expect("Failed to read config file");
    let config: Config = serde_yaml::from_str(&yaml_contents).expect("Failed to parse config file");

    config
}

pub fn generate_schema() {
    let schema = schema_for!(Config);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}

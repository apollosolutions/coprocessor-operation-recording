use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Config {
    #[serde(default = "default_listen")]
    pub listen: String,
    #[serde(default = "default_interval")]
    pub interval: u64,
    #[serde(default = "default_batch_size")]
    pub batch_size: u64,
}

// TODO: improve error handling
pub fn parse_config(path: &str) -> Config {
    let yaml_contents = match std::fs::read_to_string(path).unwrap_or_default() {
        contents if contents.is_empty() => {
            contents
        }
        contents => contents,
    };

    let config: Config = serde_yaml::from_str(&yaml_contents).expect("Failed to parse config file");

    config
}

pub fn generate_schema() {
    let schema = schema_for!(Config);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}

fn default_listen() -> String {
    "127.0.0.1:4000".to_string()
}

fn default_interval() -> u64 {
    5
}
fn default_batch_size() -> u64 {
    0
}

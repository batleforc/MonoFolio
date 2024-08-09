use std::path::PathBuf;

use dotenvy::dotenv;
use serde::Deserialize;
use tool_tracing::tracing_kind::Tracing;
use tracing::info;

const API_PORT: &str = "API_PORT";
const CONTENT_PATH: &str = "CONTENT_PATH";
const ENV: &str = "ENV";

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub env: String,
    pub content_path: String,
    pub tracing: Vec<Tracing>,
}

impl Config {
    pub fn get_name(&self) -> String {
        format!("monofolio-back-{}", self.env)
    }
}

fn override_config_with_env(config: Config) -> Config {
    let mut config = config;
    if let Ok(port) = std::env::var(API_PORT) {
        if let Ok(port) = port.parse::<u16>() {
            config.port = port;
        }
    }
    if let Ok(env) = std::env::var(ENV) {
        config.env = env;
    }
    if let Ok(content_path) = std::env::var(CONTENT_PATH) {
        config.content_path = content_path;
    }
    config
}

fn parse_config_from_file(path_buf: PathBuf) -> Config {
    let file = std::fs::File::open(path_buf).expect("file should open read only");
    let reader = std::io::BufReader::new(file);
    serde_yaml::from_reader(reader).expect("file should be proper YAML")
}

pub fn parse_config(path_buf: PathBuf) -> Config {
    let config = parse_config_from_file(path_buf);
    override_config_with_env(config)
}

pub fn parse_local_config() -> Config {
    let mut path_buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path_buf.push("../../content/config.yaml");
    match dotenv() {
        Ok(_) => info!("Loaded .env file"),
        Err(err) => println!("No .env file found: {:?}", err),
    }
    parse_config(path_buf)
}

#[allow(dead_code)]
pub fn parse_test_config() -> Config {
    let mut path_buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path_buf.push("../../test_dataset/content/config.yaml");
    println!("{:?}", path_buf);
    parse_config(path_buf)
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn test_parse_config() {
        let config = parse_test_config();
        assert_eq!(config.port, 5437);
        assert_eq!(config.content_path, "./content");
        assert_eq!(config.get_name(), "monofolio-back-development");
        let config = Config {
            port: 8080,
            content_path: "content".to_string(),
            tracing: vec![],
            env: "development".to_string(),
        };
        env::set_var(API_PORT, "8081");
        env::set_var(CONTENT_PATH, "content2");
        env::set_var(ENV, "testing");
        let config = override_config_with_env(config);
        assert_eq!(config.port, 8081);
        assert_eq!(config.content_path, "content2");
        assert_eq!(config.env, "testing");
        assert_eq!(config.get_name(), "monofolio-back-testing");
        let _local = parse_local_config();
        assert!(true);
    }
}

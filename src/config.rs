use serde::Deserialize;
use config::{Config, ConfigError, Environment, File};
use clap::Parser;
use std::collections::HashMap;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Host to listen on
    #[arg(long, env = "SERVER_HOST")]
    pub host: Option<String>,

    /// Port to listen on
    #[arg(long, env = "SERVER_PORT")]
    pub port: Option<u16>,

    /// Config file path
    #[arg(long, short, default_value = "config.toml")]
    pub config: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".into(),
            port: 8080,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ModelSpec {
    pub path: String,
    pub tokenizer_path: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    #[serde(default)]
    pub models: HashMap<String, ModelSpec>,
    #[serde(default)]
    pub aliases: HashMap<String, String>,
    #[serde(default = "default_model")]
    pub default_model: String,
}

fn default_model() -> String {
    "nllb".to_string()
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            models: HashMap::new(),
            aliases: HashMap::new(),
            default_model: default_model(),
        }
    }
}

impl AppConfig {
    pub fn load(args: Option<Args>) -> Result<Self, ConfigError> {
        let config_path = args.as_ref().map(|a| a.config.as_str()).unwrap_or("config.toml");

        let mut builder = Config::builder()
            // Start with defaults
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 8080)?
            .set_default("default_model", "nllb")?
            // Add config file
            .add_source(File::with_name(config_path).required(false))
            // Add environment variables (e.g. SERVER_PORT)
            .add_source(Environment::default().separator("_"));

        // Add CLI arguments if provided
        if let Some(args) = args {
            if let Some(host) = args.host {
                builder = builder.set_override("server.host", host)?;
            }
            if let Some(port) = args.port {
                builder = builder.set_override("server.port", port)?;
            }
        }

        builder.build()?.try_deserialize()
    }
}

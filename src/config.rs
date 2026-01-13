use clap::Parser;
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
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
    pub model_type: String, // e.g. "t5", "nllb"
    pub tokenizer_path: Option<String>,
    pub target_lang: Option<String>,
    pub device: Option<String>,
    pub device_indices: Option<Vec<i32>>,
    pub beam_size: Option<usize>,
    pub repetition_penalty: Option<f32>,
    pub no_repeat_ngram_size: Option<usize>,
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
    #[serde(default = "default_target_lang")]
    pub target_lang: String,
    #[serde(default = "default_device")]
    pub device: String,
    #[serde(default = "default_device_indices")]
    pub device_indices: Vec<i32>,
    #[serde(default = "default_beam_size")]
    pub beam_size: usize,
    #[serde(default = "default_repetition_penalty")]
    pub repetition_penalty: f32,
    #[serde(default = "default_no_repeat_ngram_size")]
    pub no_repeat_ngram_size: usize,
}

fn default_model() -> String {
    "nllb".to_string()
}

fn default_target_lang() -> String {
    "eng_Latn".to_string()
}

fn default_device() -> String {
    "cpu".to_string()
}

fn default_device_indices() -> Vec<i32> {
    vec![0]
}

fn default_beam_size() -> usize {
    5
}

fn default_repetition_penalty() -> f32 {
    1.2
}

fn default_no_repeat_ngram_size() -> usize {
    0
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            models: HashMap::new(),
            aliases: HashMap::new(),
            default_model: default_model(),
            target_lang: default_target_lang(),
            device: default_device(),
            device_indices: default_device_indices(),
            beam_size: default_beam_size(),
            repetition_penalty: default_repetition_penalty(),
            no_repeat_ngram_size: default_no_repeat_ngram_size(),
        }
    }
}

impl AppConfig {
    pub fn load(args: Option<Args>) -> Result<Self, ConfigError> {
        let config_path = args
            .as_ref()
            .map(|a| a.config.as_str())
            .unwrap_or("config.toml");

        let mut builder = Config::builder()
            // Start with defaults
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 8080)?
            .set_default("default_model", "nllb")?
            .set_default("target_lang", "eng_Latn")?
            .set_default("device", "cpu")?
            .set_default("beam_size", 5)?
            .set_default("repetition_penalty", 1.2)?
            .set_default("no_repeat_ngram_size", 0)?
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

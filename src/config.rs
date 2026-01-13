use clap::Parser;
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Host to listen on
    #[arg(long, env = "SERVER_HOST")]
    pub host: Option<String>,

    /// Port to listen on
    #[arg(long, env = "SERVER_PORT")]
    pub port: Option<u16>,
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

#[derive(Debug, Deserialize, Clone, Default)]
pub struct AppConfig {
    pub server: ServerConfig,
}

impl AppConfig {
    pub fn load(args: Option<Args>) -> Result<Self, ConfigError> {
        let mut builder = Config::builder()
            // Start with defaults
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 8080)?
            // Add config file
            .add_source(File::with_name("config").required(false))
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

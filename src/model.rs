use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use snafu::{prelude::*, Location};
use ct2rs::{Translator, Config, TranslationOptions};
use ct2rs::tokenizers::auto::Tokenizer as AutoTokenizer;
use crate::config::AppConfig;

#[derive(Debug, Snafu)]
pub enum ModelError {
    #[snafu(display("Failed to load model from {} at {}: {}", path.display(), location, source))]
    LoadError { path: PathBuf, source: anyhow::Error, #[snafu(implicit)] location: Location },
    #[snafu(display("Model not found: {:?} at {}", model_name, location))]
    NotFound { model_name: String, #[snafu(implicit)] location: Location },
    #[snafu(display("Inference failed at {}: {}", location, source))]
    InferenceError { source: anyhow::Error, #[snafu(implicit)] location: Location },
    #[snafu(display("Model configuration not found for '{}' at {}", model_name, location))]
    ConfigNotFound { model_name: String, #[snafu(implicit)] location: Location },
}

pub struct ModelManager {
    translators: Arc<RwLock<HashMap<String, Arc<Translator<AutoTokenizer>>>>>,
    config: AppConfig,
}

impl ModelManager {
    pub fn new(config: AppConfig) -> Self {
        Self {
            translators: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    pub fn resolve_model_name(&self, name: &str) -> String {
        // 1. Check if it's an alias
        if let Some(real_name) = self.config.aliases.get(name) {
            return real_name.clone();
        }
        // 2. Check if it's the default request (e.g. empty or "default")? 
        // OpenAI usually requires exact names, but we can be nice.
        if name == "default" {
            return self.config.default_model.clone();
        }
        // 3. Return as is
        name.to_string()
    }

    pub async fn load_model(&self, name: &str) -> Result<(), ModelError> {
        let resolved_name = self.resolve_model_name(name);
        
        // Check if already loaded
        if self.translators.read().await.contains_key(&resolved_name) {
            return Ok(());
        }

        // Get config
        let spec = self.config.models.get(&resolved_name)
            .context(ConfigNotFoundSnafu { model_name: resolved_name.clone() })?;

        let model_path = PathBuf::from(&spec.path);
        // Tokenizer path is usually same as model path if not specified
        // ct2rs AutoTokenizer::new takes a path to look for tokenizer files
        let tokenizer_path = spec.tokenizer_path.as_ref()
            .map(PathBuf::from)
            .unwrap_or_else(|| model_path.clone());

        // CTranslate2 loading is blocking
        let model_path_clone = model_path.clone();
        let _tokenizer_path_clone = tokenizer_path.clone(); // In case we need it later explicitly

        let translator = tokio::task::spawn_blocking(move || {
            Translator::new(model_path_clone, &Config::default())
        })
        .await
        .map_err(|e| anyhow::anyhow!("Join error: {}", e))
        .context(LoadSnafu { path: model_path.clone() })?
        .context(LoadSnafu { path: model_path.clone() })?;

        let mut translators = self.translators.write().await;
        translators.insert(resolved_name, Arc::new(translator));
        Ok(())
    }

    pub async fn get_translator(&self, name: &str) -> Result<Arc<Translator<AutoTokenizer>>, ModelError> {
        let resolved_name = self.resolve_model_name(name);
        let translators = self.translators.read().await;
        translators.get(&resolved_name).cloned().context(NotFoundSnafu { model_name: resolved_name })
    }

    pub async fn generate(&self, name: &str, prompts: Vec<String>) -> Result<Vec<String>, ModelError> {
        let translator = self.get_translator(name).await?;

        tokio::task::spawn_blocking(move || {
            translator.translate_batch(&prompts, &TranslationOptions::default(), None)
                .map(|results| results.into_iter().map(|(s, _)| s).collect())
        })
        .await
        .map_err(|e| anyhow::anyhow!("Join error: {}", e))
        .context(InferenceSnafu)?
        .context(InferenceSnafu)
    }
}

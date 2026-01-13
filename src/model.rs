use ct2rs::tokenizers::auto::Tokenizer as AutoTokenizer;
use ct2rs::{Config as Ct2Config, Device, TranslationOptions, Translator};
use snafu::{Location, prelude::*};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::config::AppConfig;

#[derive(Debug, Clone, Default)]
pub struct GenerationParams {
    pub target_lang: Option<String>,
    pub beam_size: Option<usize>,
    pub repetition_penalty: Option<f32>,
    pub no_repeat_ngram_size: Option<usize>,
}

#[derive(Debug, Snafu)]
pub enum ModelError {
    #[snafu(display("Failed to load model from {} at {}: {}", path.display(), location, source))]
    LoadError {
        path: PathBuf,
        source: anyhow::Error,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("Model not found: {:?} at {}", model_name, location))]
    NotFound {
        model_name: String,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("Inference failed at {}: {}", location, source))]
    InferenceError {
        source: anyhow::Error,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("Model configuration not found for '{}' at {}", model_name, location))]
    ConfigNotFound {
        model_name: String,
        #[snafu(implicit)]
        location: Location,
    },
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

    fn parse_device(device: &str) -> Device {
        match device.to_lowercase().as_str() {
            "cuda" => Device::CUDA,
            _ => Device::CPU,
        }
    }

    pub fn resolve_model_name(&self, name: &str) -> String {
        // 1. Check if it's an alias
        if let Some(real_name) = self.config.aliases.get(name) {
            return real_name.clone();
        }
        // 2. Check if it's the default request
        if name == "default" || name.is_empty() {
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
        let spec = self
            .config
            .models
            .get(&resolved_name)
            .context(ConfigNotFoundSnafu {
                model_name: resolved_name.clone(),
            })?;

        let model_path = PathBuf::from(&spec.path);

        // Resolve device settings
        let device_str = spec.device.as_ref().unwrap_or(&self.config.device);
        let device = Self::parse_device(device_str);

        let device_indices = spec
            .device_indices
            .as_ref()
            .unwrap_or(&self.config.device_indices);

        // CTranslate2 loading is blocking
        let model_path_clone = model_path.clone();
        let ct2_config = Ct2Config {
            device,
            device_indices: device_indices.clone(),
            ..Default::default()
        };

        let tokenizer_path = spec.tokenizer_path.as_ref().unwrap_or(&spec.path);
        let tokenizer = ct2rs::tokenizers::auto::Tokenizer::new(tokenizer_path)
            .map_err(|e| anyhow::anyhow!("Failed to load tokenizer: {}", e))
            .context(LoadSnafu {
                path: tokenizer_path,
            })?;

        let translator = tokio::task::spawn_blocking(move || {
            Translator::with_tokenizer(model_path_clone, tokenizer, &ct2_config)
        })
        .await
        .map_err(|e| anyhow::anyhow!("Join error: {}", e))
        .context(LoadSnafu {
            path: model_path.clone(),
        })?
        .map_err(|e| anyhow::anyhow!(e))
        .context(LoadSnafu {
            path: model_path.clone(),
        })?;

        let mut translators = self.translators.write().await;
        translators.insert(resolved_name, Arc::new(translator));
        Ok(())
    }

    pub async fn get_translator(
        &self,
        name: &str,
    ) -> Result<Arc<Translator<AutoTokenizer>>, ModelError> {
        let resolved_name = self.resolve_model_name(name);

        // 1. Check if already loaded
        {
            let translators = self.translators.read().await;
            if let Some(translator) = translators.get(&resolved_name) {
                return Ok(translator.clone());
            }
        }

        // 2. Try to load if not loaded (Lazy loading)
        tracing::info!("Lazy loading model: {}", resolved_name);
        self.load_model(&resolved_name).await?;

        // 3. Get after load
        let translators = self.translators.read().await;
        translators
            .get(&resolved_name)
            .cloned()
            .context(NotFoundSnafu {
                model_name: resolved_name,
            })
    }

    pub async fn generate(
        &self,
        name: &str,
        prompts: Vec<String>,
        params: GenerationParams,
    ) -> Result<Vec<String>, ModelError> {
        let resolved_name = self.resolve_model_name(name);
        let translator = self.get_translator(name).await?;

        // Resolve config
        let model_spec = self.config.models.get(&resolved_name);

        // 1. Target Lang
        let target_lang = params
            .target_lang
            .or_else(|| model_spec.and_then(|m| m.target_lang.clone()))
            .unwrap_or_else(|| self.config.target_lang.clone());

        // 2. Beam Size
        let beam_size = params
            .beam_size
            .or_else(|| model_spec.and_then(|m| m.beam_size))
            .unwrap_or(self.config.beam_size);

        // 3. Repetition Penalty
        let repetition_penalty = params
            .repetition_penalty
            .or_else(|| model_spec.and_then(|m| m.repetition_penalty))
            .unwrap_or(self.config.repetition_penalty);

        // 4. No Repeat Ngram Size
        let no_repeat_ngram_size = params
            .no_repeat_ngram_size
            .or_else(|| model_spec.and_then(|m| m.no_repeat_ngram_size))
            .unwrap_or(self.config.no_repeat_ngram_size);

        tokio::task::spawn_blocking(move || {
            let options = TranslationOptions {
                beam_size,
                repetition_penalty,
                no_repeat_ngram_size,
                ..Default::default()
            };

            // Replicate the prefix for each prompt in the batch
            let target_prefixes: Vec<Vec<String>> = std::iter::repeat(vec![target_lang.clone()])
                .take(prompts.len())
                .collect();

            translator
                .translate_batch_with_target_prefix(&prompts, &target_prefixes, &options, None)
                .map(|results| results.into_iter().map(|(s, _)| s).collect())
        })
        .await
        .map_err(|e| anyhow::anyhow!("Join error: {}", e))
        .context(InferenceSnafu)?
        .context(InferenceSnafu)
    }
}

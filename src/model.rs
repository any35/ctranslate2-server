use ct2rs::tokenizers::auto::Tokenizer as AutoTokenizer;
use ct2rs::{Config, TranslationOptions, Translator};
use snafu::{Location, prelude::*};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Snafu)]
pub enum ModelError {
    #[snafu(display("Failed to load model from {} at {}: {}", path.display(), location, source))]
    LoadError {
        path: PathBuf,
        source: anyhow::Error,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("Model not found: {:?} at {}", model_type, location))]
    NotFound {
        model_type: ModelType,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("Inference failed at {}: {}", location, source))]
    InferenceError {
        source: anyhow::Error,
        #[snafu(implicit)]
        location: Location,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModelType {
    NLLB,
    T5,
    Whisper,
}

pub struct ModelManager {
    translators: Arc<RwLock<HashMap<ModelType, Arc<Translator<AutoTokenizer>>>>>,
}

impl ModelManager {
    pub fn new() -> Self {
        Self {
            translators: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn load_model(&self, model_type: ModelType, path: PathBuf) -> Result<(), ModelError> {
        // CTranslate2 loading is blocking, so we use spawn_blocking
        let path_clone = path.clone();
        let translator =
            tokio::task::spawn_blocking(move || Translator::new(path_clone, &Config::default()))
                .await
                .map_err(|e| anyhow::anyhow!("Join error: {}", e))
                .context(LoadSnafu { path: path.clone() })?
                .context(LoadSnafu { path: path.clone() })?;

        let mut translators = self.translators.write().await;
        translators.insert(model_type, Arc::new(translator));
        Ok(())
    }

    pub async fn get_translator(
        &self,
        model_type: ModelType,
    ) -> Result<Arc<Translator<AutoTokenizer>>, ModelError> {
        let translators = self.translators.read().await;
        translators
            .get(&model_type)
            .cloned()
            .context(NotFoundSnafu { model_type })
    }

    pub async fn generate(
        &self,
        model_type: ModelType,
        prompts: Vec<String>,
    ) -> Result<Vec<String>, ModelError> {
        let translator = self.get_translator(model_type).await?;

        tokio::task::spawn_blocking(move || {
            translator
                .translate_batch(&prompts, &TranslationOptions::default(), None)
                .map(|results| results.into_iter().map(|(s, _)| s).collect())
        })
        .await
        .map_err(|e| anyhow::anyhow!("Join error: {}", e))
        .context(InferenceSnafu)?
        .context(InferenceSnafu)
    }
}

impl Default for ModelManager {
    fn default() -> Self {
        Self::new()
    }
}

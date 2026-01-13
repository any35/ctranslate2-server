use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use snafu::prelude::*;
use ct2rs::{Translator, Config};
use ct2rs::tokenizers::auto::Tokenizer as AutoTokenizer;

#[derive(Debug, Snafu)]
pub enum ModelError {
    #[snafu(display("Failed to load model from {}: {}", path.display(), source))]
    LoadError { path: PathBuf, source: anyhow::Error },
    #[snafu(display("Model not found: {:?}", model_type))]
    NotFound { model_type: ModelType },
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
        let translator = tokio::task::spawn_blocking(move || {
            Translator::new(path_clone, &Config::default())
        })
        .await
        .map_err(|e| ModelError::LoadError { 
            path: path.clone(), 
            source: anyhow::anyhow!("Join error: {}", e) 
        })?
        .map_err(|e| ModelError::LoadError { 
            path: path.clone(), 
            source: anyhow::anyhow!(e) 
        })?;

        let mut translators = self.translators.write().await;
        translators.insert(model_type, Arc::new(translator));
        Ok(())
    }

    pub async fn get_translator(&self, model_type: ModelType) -> Result<Arc<Translator<AutoTokenizer>>, ModelError> {
        let translators = self.translators.read().await;
        translators.get(&model_type).cloned().context(NotFoundSnafu { model_type })
    }
}

impl Default for ModelManager {
    fn default() -> Self {
        Self::new()
    }
}

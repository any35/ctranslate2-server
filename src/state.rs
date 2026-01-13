use crate::model::ModelManager;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub model_manager: Arc<ModelManager>,
}

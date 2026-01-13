use ctranslate2_server::model::{ModelManager, ModelType};
use std::path::PathBuf;

#[tokio::test]
async fn model_manager_load_error_on_invalid_path() {
    let manager = ModelManager::new();
    let result = manager.load_model(ModelType::T5, PathBuf::from("/non/existent/path")).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn model_manager_generate_error_if_not_loaded() {
    let manager = ModelManager::new();
    let result = manager.generate(ModelType::T5, vec!["Hello".into()]).await;
    assert!(result.is_err());
}
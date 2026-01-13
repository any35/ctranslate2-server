use ctranslate2_server::{model::ModelManager, config::AppConfig};

#[tokio::test]
async fn model_manager_load_error_on_invalid_path() {
    // We need to provide a config that has "t5" defined to pass the "ConfigNotFound" check
    // and fail at the actual loading step
    let mut config = AppConfig::default();
    config.models.insert("t5".to_string(), ctranslate2_server::config::ModelSpec {
        path: "/non/existent/path".to_string(),
        tokenizer_path: None,
    });
    
    let manager = ModelManager::new(config);
    let result = manager.load_model("t5").await;
    assert!(result.is_err());
    // Verify it's a LoadError, not ConfigNotFound
    let err_string = format!("{}", result.unwrap_err());
    assert!(err_string.contains("Failed to load model"));
}

#[tokio::test]
async fn model_manager_generate_error_if_not_loaded() {
    let mut config = AppConfig::default();
    config.models.insert("t5".to_string(), ctranslate2_server::config::ModelSpec {
        path: "/tmp".to_string(),
        tokenizer_path: None,
    });
    let manager = ModelManager::new(config);
    let result = manager.generate("t5", vec!["Hello".into()]).await;
    assert!(result.is_err());
}

#[test]
fn resolve_alias_works() {
    let mut config = AppConfig::default();
    config.aliases.insert("nllb-small".to_string(), "nllb".to_string());
    config.default_model = "nllb".to_string();

    let manager = ModelManager::new(config);
    
    assert_eq!(manager.resolve_model_name("nllb-small"), "nllb");
    assert_eq!(manager.resolve_model_name("default"), "nllb");
    assert_eq!(manager.resolve_model_name("other"), "other");
}

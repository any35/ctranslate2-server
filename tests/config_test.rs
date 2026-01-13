use ctranslate2_server::config::AppConfig;

#[test]
fn default_config_is_correct() {
    let config = AppConfig::default();
    assert_eq!(config.server.port, 8080);
    assert_eq!(config.server.host, "0.0.0.0");
}

#[test]
fn env_vars_override_defaults() {
    unsafe { std::env::set_var("SERVER_PORT", "9090") };
    let config = AppConfig::load(None).unwrap();
    assert_eq!(config.server.port, 9090);
    // Cleanup
    unsafe { std::env::remove_var("SERVER_PORT") };
}

#[test]
fn cli_args_override_everything() {
    use ctranslate2_server::config::Args;
    let args = Args {
        host: None,
        port: Some(7070),
        config: "config.toml".into(),
    };
    unsafe { std::env::set_var("SERVER_PORT", "9090") };
    let config = AppConfig::load(Some(args)).unwrap();
    assert_eq!(config.server.port, 7070);
    // Cleanup
    unsafe { std::env::remove_var("SERVER_PORT") };
}

use std::fs;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut config = String::from("# Auto-generated configuration\n\n");

    config.push_str("[server]\n");
    config.push_str("host = \"0.0.0.0\"\n");
    config.push_str("port = 8080\n\n");

    config.push_str("default_model = \"nllb\"\n");

    config.push_str("target_lang = \"eng_Latn\"\n\n");

    config.push_str("[aliases]\n");

    // We will populate this dynamically if we find suitable models
    let mut nllb_found = None;

    // Buffer model entries to write them after aliases (or just write aliases at the end,
    // but TOML tables usually come last).
    // Actually, in TOML, [table] sections come after key-values.
    // So we should build the models string separately.
    let mut models_config = String::from("\n[models]\n");

    for entry in fs::read_dir("./models")? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                let model_type = if name.contains("t5") {
                    "t5"
                } else if name.contains("nllb") {
                    if nllb_found.is_none() {
                        nllb_found = Some(name.to_string());
                    }
                    "nllb"
                } else {
                    "unknown"
                };

                models_config.push_str(&format!("[models.\"{}\"]\n", name));
                models_config.push_str(&format!("path = \"{}\"\n", path.display()));
                models_config.push_str(&format!("model_type = \"{}\"\n", model_type));
                // models_config.push_str("target_lang = \"zho_Hans\"\n\n");
                models_config.push_str("\n");
            }
        }
    }
    if let Some(nllb_name) = nllb_found {
        config.push_str(&format!("\"nllb\" = \"{}\"\n", nllb_name));
    }

    config.push_str(&models_config);

    let mut file = fs::File::create("config.toml")?;
    file.write_all(config.as_bytes())?;
    println!("Generated config.toml");

    Ok(())
}

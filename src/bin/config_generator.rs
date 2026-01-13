use std::fs;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut config = String::from("# Auto-generated configuration\n\n");

    config.push_str("[server]\n");
    config.push_str("host = \"0.0.0.0\"\n");
    config.push_str("port = 8080\n\n");

    config.push_str("default_model = \"nllb\"\n\n");

    config.push_str("[aliases]\n");
    config.push_str("# nllb-large = \"nllb\"\n\n");

    config.push_str("[models]\n");

    for entry in fs::read_dir("./models")? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                let model_type = if name.contains("t5") {
                    "t5"
                } else if name.contains("nllb") {
                    "nllb"
                } else {
                    "unknown"
                };

                config.push_str(&format!("[models.\"{}\"]\n", name));
                config.push_str(&format!("path = \"{}\"\n", path.display()));
                config.push_str(&format!("model_type = \"{}\"\n\n", model_type));
            }
        }
    }

    let mut file = fs::File::create("config.toml")?;
    file.write_all(config.as_bytes())?;

    println!("Generated config.toml");

    Ok(())
}

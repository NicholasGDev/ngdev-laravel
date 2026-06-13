use crate::cli::PdvArgs;
use super::templates;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn generate(args: &PdvArgs) -> Result<()> {
    let base_ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    if !args.models_only {
        generate_migrations(base_ts)?;
    }

    if !args.migrations_only {
        generate_models()?;
    }

    Ok(())
}

fn generate_migrations(base_ts: u64) -> Result<()> {
    fs::create_dir_all("database/migrations")?;

    for (i, (suffix, content)) in templates::migrations().iter().enumerate() {
        let ts = base_ts + i as u64;
        let path = PathBuf::from(format!("database/migrations/{}_{}.php", ts, suffix));
        fs::write(&path, content)?;
        println!("Created: {}", path.display());
    }

    Ok(())
}

fn generate_models() -> Result<()> {
    fs::create_dir_all("app/Models")?;

    for (name, content) in templates::models() {
        let path = PathBuf::from(format!("app/Models/{}.php", name));
        fs::write(&path, content)?;
        println!("Created: {}", path.display());
    }

    Ok(())
}

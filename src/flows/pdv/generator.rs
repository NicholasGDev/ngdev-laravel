use crate::cli::PdvArgs;
use super::templates;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn generate(args: &PdvArgs) -> Result<()> {
    let root = PathBuf::from(&args.project_root);
    fs::create_dir_all(&root)?;

    let base_ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    if !args.models_only {
        generate_migrations(&root, base_ts)?;
    }

    if !args.migrations_only {
        generate_models(&root)?;
    }

    Ok(())
}

fn generate_migrations(root: &PathBuf, base_ts: u64) -> Result<()> {
    let dir = root.join("database/migrations");
    fs::create_dir_all(&dir)?;

    for (i, (suffix, content)) in templates::migrations().iter().enumerate() {
        let ts = base_ts + i as u64;
        let path = dir.join(format!("{}_{}.php", ts, suffix));
        fs::write(&path, content)?;
        println!("Created: {}", path.display());
    }

    Ok(())
}

fn generate_models(root: &PathBuf) -> Result<()> {
    let dir = root.join("app/Models");
    fs::create_dir_all(&dir)?;

    for (name, content) in templates::models() {
        let path = dir.join(format!("{}.php", name));
        fs::write(&path, content)?;
        println!("Created: {}", path.display());
    }

    Ok(())
}

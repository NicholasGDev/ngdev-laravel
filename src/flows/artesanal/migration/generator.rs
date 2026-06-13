use crate::cli::MigrationArgs;
use super::templates;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub fn generate(args: &MigrationArgs) -> Result<()> {
    let timestamp = chrono_like_timestamp();
    let filename = format!("{}_{}", timestamp, args.name);
    let table = args
        .table
        .clone()
        .unwrap_or_else(|| extract_table_from_name(&args.name));

    let content = templates::render(&args.name, &table)?;
    let root = PathBuf::from(&args.project_root);
    let dir = root.join("database/migrations");
    fs::create_dir_all(&dir)?;
    let path = dir.join(format!("{}.php", filename));
    fs::write(&path, &content)?;
    println!("Created: {}", path.display());

    Ok(())
}

fn chrono_like_timestamp() -> String {
    // Simple timestamp using std — replace with chrono if needed
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    // Format: YYYY_MM_DD_HHmmss (approximate, not calendar-accurate)
    format!("{}", secs)
}

fn extract_table_from_name(name: &str) -> String {
    // e.g. "create_users_table" -> "users"
    name.replace("create_", "").replace("_table", "")
}

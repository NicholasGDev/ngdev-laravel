use crate::cli::MigrationArgs;
use super::templates;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub fn generate(args: &MigrationArgs) -> Result<()> {
    let timestamp = laravel_timestamp();
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

fn laravel_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let total = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    let days     = total / 86400;
    let rem      = total % 86400;
    let hh       = rem / 3600;
    let mm       = (rem % 3600) / 60;
    let ss       = rem % 60;

    // Euclidean affine algorithm — Howard Hinnant
    let z   = days + 719_468;
    let era = if z >= 0 { z / 146_097 } else { (z - 146_096) / 146_097 };
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let year_base = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp  = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let mon = if mp < 10 { mp + 3 } else { mp - 9 };
    let yr  = if mon <= 2 { year_base + 1 } else { year_base };

    format!("{:04}_{:02}_{:02}_{:02}{:02}{:02}", yr, mon, day, hh, mm, ss)
}

fn extract_table_from_name(name: &str) -> String {
    // e.g. "create_users_table" -> "users"
    name.replace("create_", "").replace("_table", "")
}

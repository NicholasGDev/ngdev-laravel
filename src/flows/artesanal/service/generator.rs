use crate::cli::ServiceArgs;
use super::templates;
use anyhow::Result;
use heck::ToPascalCase;
use std::fs;
use std::path::PathBuf;

pub fn generate(args: &ServiceArgs) -> Result<()> {
    let name  = args.name.to_pascal_case();
    let model = args.model.to_pascal_case();

    let content = templates::render(&name, &model);

    let root = PathBuf::from(&args.project_root);
    let dir  = root.join("app/Services");
    fs::create_dir_all(&dir)?;
    let path = dir.join(format!("{}Service.php", name));
    fs::write(&path, content)?;
    println!("  Created: {}", path.display());

    Ok(())
}

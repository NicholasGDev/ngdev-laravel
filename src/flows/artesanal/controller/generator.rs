use crate::cli::ControllerArgs;
use super::templates;
use anyhow::Result;
use heck::ToPascalCase;
use std::fs;
use std::path::PathBuf;

pub fn generate(args: &ControllerArgs) -> Result<()> {
    let name = args.name.to_pascal_case();
    let model = args.model.as_deref().map(|m| m.to_pascal_case());

    let content = if args.resource {
        templates::render_resource(&name, model.as_deref())?
    } else {
        templates::render_plain(&name)?
    };

    let root = PathBuf::from(&args.project_root);
    let dir = root.join("app/Http/Controllers");
    fs::create_dir_all(&dir)?;
    let path = dir.join(format!("{}.php", name));
    fs::write(&path, content)?;
    println!("Created: {}", path.display());

    Ok(())
}

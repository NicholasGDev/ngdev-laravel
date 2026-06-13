use crate::cli::ControllerArgs;
use crate::templates;
use anyhow::Result;
use heck::ToPascalCase;
use std::fs;
use std::path::PathBuf;

pub fn generate(args: &ControllerArgs) -> Result<()> {
    let name = args.name.to_pascal_case();
    let model = args.model.as_deref().map(|m| m.to_pascal_case());

    let content = if args.resource {
        templates::controller::render_resource(&name, model.as_deref())?
    } else {
        templates::controller::render_plain(&name)?
    };

    let path = PathBuf::from(format!("app/Http/Controllers/{}.php", name));

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&path, content)?;
    println!("Created: {}", path.display());

    Ok(())
}

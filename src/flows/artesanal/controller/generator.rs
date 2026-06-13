use crate::cli::ControllerArgs;
use super::templates;
use anyhow::Result;
use heck::ToPascalCase;
use std::fs;
use std::path::PathBuf;

pub fn generate(args: &ControllerArgs) -> Result<()> {
    let name  = args.name.to_pascal_case();
    let model = args.model.as_deref().map(|m| m.to_pascal_case());

    // Base name para o Service: prefere o model, cai no nome sem sufixo "Controller".
    let base_name: String = model.clone()
        .unwrap_or_else(|| name.trim_end_matches("Controller").to_string());

    // Gera o Service antes do Controller quando solicitado.
    if args.service {
        crate::flows::artesanal::service::generator::generate(&crate::cli::ServiceArgs {
            name:         base_name.clone(),
            model:        base_name.clone(),
            project_root: args.project_root.clone(),
        })?;
    }

    let content: String = if args.service {
        templates::render_resource_with_service(&name, &base_name)
    } else if args.resource {
        templates::render_resource(&name, model.as_deref())?
    } else {
        templates::render_plain(&name)?
    };

    let root = PathBuf::from(&args.project_root);
    let dir  = root.join("app/Http/Controllers");
    fs::create_dir_all(&dir)?;
    let path = dir.join(format!("{}.php", name));
    fs::write(&path, content)?;
    println!("  Created: {}", path.display());

    Ok(())
}

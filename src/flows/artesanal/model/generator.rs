use crate::cli::ModelArgs;
use super::templates;
use anyhow::Result;
use heck::{ToPascalCase, ToSnakeCase};
use std::fs;
use std::path::PathBuf;

pub fn generate(args: &ModelArgs) -> Result<()> {
    let name = args.name.to_pascal_case();
    let table = name.to_snake_case() + "s";
    let root = PathBuf::from(&args.project_root);

    let content = templates::render(&name, &table)?;
    let dir = root.join("app/Models");
    fs::create_dir_all(&dir)?;
    let path = dir.join(format!("{}.php", name));
    fs::write(&path, &content)?;
    println!("Created: {}", path.display());

    if args.migration {
        let migration_name = format!("create_{}_table", table);
        crate::flows::artesanal::migration::generator::generate(&crate::cli::MigrationArgs {
            name: migration_name,
            table: Some(table),
            project_root: args.project_root.clone(),
        })?;
    }

    if args.controller {
        crate::flows::artesanal::controller::generator::generate(&crate::cli::ControllerArgs {
            name: format!("{}Controller", name),
            resource: true,
            model: Some(name),
            project_root: args.project_root.clone(),
        })?;
    }

    Ok(())
}

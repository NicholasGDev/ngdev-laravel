use crate::cli::ModelArgs;
use crate::templates;
use anyhow::Result;
use heck::{ToPascalCase, ToSnakeCase};
use std::fs;
use std::path::PathBuf;

pub fn generate(args: &ModelArgs) -> Result<()> {
    let name = args.name.to_pascal_case();
    let table = name.to_snake_case() + "s";

    let content = templates::model::render(&name, &table)?;
    let path = PathBuf::from(format!("app/Models/{}.php", name));

    write_file(&path, &content)?;
    println!("Created: {}", path.display());

    if args.migration {
        let migration_name = format!("create_{}_table", table);
        crate::generators::migration::generate(&crate::cli::MigrationArgs {
            name: migration_name,
            table: Some(table),
        })?;
    }

    if args.controller {
        crate::generators::controller::generate(&crate::cli::ControllerArgs {
            name: format!("{}Controller", name),
            resource: true,
            model: Some(name),
        })?;
    }

    Ok(())
}

fn write_file(path: &PathBuf, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)?;
    Ok(())
}

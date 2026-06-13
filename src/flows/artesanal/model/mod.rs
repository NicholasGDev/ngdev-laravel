pub mod generator;
pub mod templates;

use anyhow::Result;
use console::style;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};

pub fn run(theme: &ColorfulTheme) -> Result<()> {
    println!("{}", style("  [ Criar Model ]").yellow().bold());
    println!();

    let nome: String = Input::with_theme(theme)
        .with_prompt("  Nome do Model (ex: User, BlogPost)")
        .interact_text()?;

    let com_migration = Confirm::with_theme(theme)
        .with_prompt("  Gerar Migration tambem?")
        .default(false)
        .interact()?;

    let com_controller = Confirm::with_theme(theme)
        .with_prompt("  Gerar Controller tambem?")
        .default(false)
        .interact()?;

    let project_root: String = Input::with_theme(theme)
        .with_prompt("  Diretorio raiz do projeto Laravel")
        .default(".".to_string())
        .interact_text()?;

    generator::generate(&crate::cli::ModelArgs {
        name: nome,
        migration: com_migration,
        controller: com_controller,
        project_root,
    })?;

    Ok(())
}

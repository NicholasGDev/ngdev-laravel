pub mod generator;
pub mod templates;

use anyhow::Result;
use console::style;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};

pub fn run(theme: &ColorfulTheme) -> Result<()> {
    println!("{}", style("  [ Criar Model ]").yellow().bold());
    println!();

    let project_root: String = Input::with_theme(theme)
        .with_prompt("  Caminho absoluto do projeto Laravel (ex: /home/user/meu-projeto)")
        .interact_text()?;

    let nome: String = Input::with_theme(theme)
        .with_prompt("  Nome do Model (ex: User, BlogPost)")
        .interact_text()?;

    let com_migration = Confirm::with_theme(theme)
        .with_prompt("  Gerar Migration?")
        .default(true)
        .interact()?;

    let com_service = Confirm::with_theme(theme)
        .with_prompt("  Gerar Service (MVCS)?")
        .default(true)
        .interact()?;

    let com_controller = Confirm::with_theme(theme)
        .with_prompt("  Gerar Controller?")
        .default(true)
        .interact()?;

    generator::generate(&crate::cli::ModelArgs {
        name: nome,
        migration: com_migration,
        controller: com_controller,
        service: com_service,
        project_root,
    })?;

    Ok(())
}

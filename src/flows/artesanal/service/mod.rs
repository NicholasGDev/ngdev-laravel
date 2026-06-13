pub mod generator;
pub mod templates;

use anyhow::Result;
use console::style;
use dialoguer::{theme::ColorfulTheme, Input};
use heck::ToPascalCase;

pub fn run(theme: &ColorfulTheme) -> Result<()> {
    println!("{}", style("  [ Criar Service ]").yellow().bold());
    println!();

    let project_root: String = Input::with_theme(theme)
        .with_prompt("  Caminho absoluto do projeto Laravel (ex: /home/user/meu-projeto)")
        .interact_text()?;

    let nome: String = Input::with_theme(theme)
        .with_prompt("  Nome do Service (ex: User, Order) — sem sufixo 'Service'")
        .interact_text()?;

    let model_nome: String = Input::with_theme(theme)
        .with_prompt("  Model a injetar (deixe em branco para usar o mesmo nome)")
        .allow_empty(true)
        .interact_text()?;

    let model = if model_nome.trim().is_empty() {
        nome.trim().to_pascal_case()
    } else {
        model_nome.trim().to_pascal_case()
    };

    generator::generate(&crate::cli::ServiceArgs {
        name: nome,
        model,
        project_root,
    })?;

    Ok(())
}

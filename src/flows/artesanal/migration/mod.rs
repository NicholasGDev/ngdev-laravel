pub mod generator;
pub mod templates;

use anyhow::Result;
use console::style;
use dialoguer::{theme::ColorfulTheme, Input};

pub fn run(theme: &ColorfulTheme) -> Result<()> {
    println!("{}", style("  [ Criar Migration ]").yellow().bold());
    println!();

    let nome: String = Input::with_theme(theme)
        .with_prompt("  Nome da Migration (ex: create_users_table)")
        .interact_text()?;

    let tabela: String = Input::with_theme(theme)
        .with_prompt("  Nome da tabela (deixe em branco para detectar automaticamente)")
        .allow_empty(true)
        .interact_text()?;

    let project_root: String = Input::with_theme(theme)
        .with_prompt("  Caminho absoluto do projeto Laravel (ex: /home/user/meu-projeto)")
        .interact_text()?;

    generator::generate(&crate::cli::MigrationArgs {
        name: nome,
        table: if tabela.trim().is_empty() { None } else { Some(tabela) },
        project_root,
    })?;

    Ok(())
}

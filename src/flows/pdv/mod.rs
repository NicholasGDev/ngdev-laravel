pub mod generator;
pub mod templates;

use anyhow::Result;
use console::style;
use crate::flows::deps;
use dialoguer::{theme::ColorfulTheme, Input, Select};

pub fn run(theme: &ColorfulTheme) -> Result<()> {
    println!("{}", style("  [ Scaffold PDV ]").yellow().bold());
    println!();

    let opcoes = vec![
        "Gerar Migrations e Models",
        "Gerar somente Migrations",
        "Gerar somente Models",
    ];

    let selecao = Select::with_theme(theme)
        .with_prompt("  O que gerar para o PDV?")
        .items(&opcoes)
        .default(0)
        .interact()?;

    let project_root: String = Input::with_theme(theme)
        .with_prompt("  Caminho absoluto do projeto Laravel (ex: /home/user/meu-projeto)")
        .interact_text()?;

    deps::verify_all()?;

    generator::generate(&crate::cli::PdvArgs {
        migrations_only: selecao == 1,
        models_only: selecao == 2,
        project_root,
    })?;

    Ok(())
}

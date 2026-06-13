pub mod generator;
pub mod templates;

use anyhow::Result;
use console::style;
use dialoguer::{theme::ColorfulTheme, Select};

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

    generator::generate(&crate::cli::PdvArgs {
        migrations_only: selecao == 1,
        models_only: selecao == 2,
    })?;

    Ok(())
}

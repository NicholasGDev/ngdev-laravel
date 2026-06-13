pub mod generator;
pub mod templates;

use anyhow::Result;
use console::style;
use dialoguer::{theme::ColorfulTheme, Input};

pub fn run(theme: &ColorfulTheme) -> Result<()> {
    println!("{}", style("  [ Scaffold ERP Logística Reversa — MVCS ]").yellow().bold());
    println!();
    println!("  {}", style("Gera Models + Services + Controllers + Migrations").dim());
    println!("  {}", style("Entidades: Seguradora · Transportadora · Segurado · Apolice").dim());
    println!("  {}", style("           Sinistro · OrdemColeta (+ tracking) · LaudoTriagem").dim());
    println!("  {}", style("Padrão: App\\Models / App\\Services / App\\Http\\Controllers\\Logistica").dim());
    println!();

    let project_root: String = Input::with_theme(theme)
        .with_prompt("  Caminho absoluto do projeto Laravel (ex: /home/user/meu-logistica)")
        .interact_text()?;

    println!();
    println!("  {}", style("Gerando estrutura MVCS...").yellow().bold());
    println!();

    generator::generate(project_root.trim_end_matches('/'))?;

    Ok(())
}

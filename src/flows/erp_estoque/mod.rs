pub mod generator;
pub mod templates;

use anyhow::Result;
use console::style;
use dialoguer::{theme::ColorfulTheme, Input};

pub fn run(theme: &ColorfulTheme) -> Result<()> {
    println!("{}", style("  [ Scaffold ERP Estoque — MVCS ]").yellow().bold());
    println!();
    println!("  {}", style("Gera Models + Services + Controllers + Migrations").dim());
    println!("  {}", style("Entidades: Armazem · Fornecedor · Produto · Lote").dim());
    println!("  {}", style("           PedidoCompra · MovimentacaoEstoque (Kardex) · Inventario").dim());
    println!("  {}", style("Padrão: App\\Models / App\\Services / App\\Http\\Controllers\\Estoque").dim());
    println!();

    let project_root: String = Input::with_theme(theme)
        .with_prompt("  Caminho absoluto do projeto Laravel (ex: /home/user/meu-erp)")
        .interact_text()?;

    println!();
    println!("  {}", style("Gerando estrutura MVCS...").yellow().bold());
    println!();

    generator::generate(project_root.trim_end_matches('/'))?;

    Ok(())
}

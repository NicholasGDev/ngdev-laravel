pub mod generator;
pub mod templates;

use anyhow::Result;
use console::style;
use dialoguer::{theme::ColorfulTheme, Input, Select};

pub fn run(theme: &ColorfulTheme) -> Result<()> {
    println!("{}", style("  [ Scaffold ERP Estoque DDD ]").yellow().bold());
    println!();
    println!("  {}", style("Gera 6 Contexts + 10 Migrations + 10 Models + UseCases Kardex + Manager JSON").dim());
    println!("  {}", style("Entidades: Armazem · Fornecedor · Produto · Lote").dim());
    println!("  {}", style("           PedidoCompra · MovimentacaoEstoque (Kardex) · Inventario").dim());
    println!();

    let base_path: String = Input::with_theme(theme)
        .with_prompt("  Diretorio base dos Contexts")
        .default("back/app/Contexts".to_string())
        .interact_text()?;

    let namespace_base: String = Input::with_theme(theme)
        .with_prompt("  Namespace base PHP")
        .default("App\\Contexts".to_string())
        .interact_text()?;

    let migration_path: String = Input::with_theme(theme)
        .with_prompt("  Diretorio das Migrations")
        .default("database/migrations".to_string())
        .interact_text()?;

    let tenant_id: String = Input::with_theme(theme)
        .with_prompt("  Tenant ID (config do manager)")
        .default("corp-estoque-001".to_string())
        .interact_text()?;

    let methods = vec![
        "CUSTO_MEDIO_PONDERADO  (Custo Medio — padrao)",
        "PEPS                   (Primeiro a Entrar, Primeiro a Sair)",
    ];
    let method_idx = Select::with_theme(theme)
        .with_prompt("  Metodo de custeio")
        .items(&methods)
        .default(0)
        .interact()?;

    let valuation_method = if method_idx == 0 { "CUSTO_MEDIO_PONDERADO" } else { "PEPS" };

    println!();
    println!("  {}", style("Gerando estrutura completa...").yellow().bold());
    println!();

    generator::generate(&generator::EstoqueOptions {
        base_path,
        namespace_base,
        migration_path,
        tenant_id,
        valuation_method: valuation_method.to_string(),
    })?;

    Ok(())
}

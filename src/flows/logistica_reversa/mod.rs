pub mod generator;
pub mod templates;

use anyhow::Result;
use console::style;
use crate::flows::deps;
use dialoguer::{theme::ColorfulTheme, Input};

pub fn run(theme: &ColorfulTheme) -> Result<()> {
    println!("{}", style("  [ Scaffold Logistica Reversa de Sinistros ]").yellow().bold());
    println!();
    println!("  {}", style("Gera 7 Contexts DDD + 10 Migrations + 10 Eloquent Models + Manager JSON").dim());
    println!("  {}", style("Entidades: Seguradora · Transportadora · Segurado · Apolice").dim());
    println!("  {}", style("           Sinistro · OrdemColeta · LaudoTriagem").dim());
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

    let erp_id: String = Input::with_theme(theme)
        .with_prompt("  ERP ID (tenant config)")
        .default("revlog-core-01".to_string())
        .interact_text()?;

    let company_name: String = Input::with_theme(theme)
        .with_prompt("  Nome da empresa")
        .default("Reversa Express Log".to_string())
        .interact_text()?;

    let warehouse_id: String = Input::with_theme(theme)
        .with_prompt("  ID do CD (warehouse)")
        .default("CD-SP-01".to_string())
        .interact_text()?;

    let project_root: String = Input::with_theme(theme)
        .with_prompt("  Caminho absoluto do projeto Laravel (ex: /home/user/meu-projeto)")
        .interact_text()?;
    let root = project_root.trim_end_matches('/');
    let base_path = if base_path.starts_with('/') { base_path }
                    else { format!("{}/{}", root, base_path) };
    let migration_path = if migration_path.starts_with('/') { migration_path }
                         else { format!("{}/{}", root, migration_path) };

    println!();
    println!("  {}", style("Gerando estrutura completa...").yellow().bold());
    println!();

    deps::copy_laravel_base(root)?;

    generator::generate(&generator::LogisticaReversaOptions {
        project_root: root.to_string(),
        base_path,
        namespace_base,
        migration_path,
        erp_id,
        company_name,
        warehouse_id,
    })?;

    Ok(())
}

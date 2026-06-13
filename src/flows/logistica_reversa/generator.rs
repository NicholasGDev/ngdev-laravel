use crate::flows::context::generator::{generate as gen_context, ContextOptions};
use crate::flows::docker;
use super::templates as tmpl;
use anyhow::Result;
use console::style;
use std::fs;
use std::path::{Path, PathBuf};

pub struct LogisticaReversaOptions {
    pub project_root: String,
    pub base_path: String,
    pub namespace_base: String,
    pub migration_path: String,
    pub erp_id: String,
    pub company_name: String,
    pub warehouse_id: String,
}

/// Gera toda a estrutura DDD + migrations + models para Logística Reversa de Sinistros
pub fn generate(opts: &LogisticaReversaOptions) -> Result<()> {
    let ns = &opts.namespace_base;
    let base = &opts.base_path;

    println!();
    println!("  {}", style("[ 1/5 ] Gerando DDD Contexts...").cyan().bold());
    println!();

    // ── 7 Bounded Contexts ───────────────────────────────────────────────────

    let contexts: Vec<(&str, &str, Vec<&str>)> = vec![
        (
            "Seguradora",
            "seguradoras",
            vec!["consultar", "detalhar", "criar", "alterar"],
        ),
        (
            "Transportadora",
            "transportadoras",
            vec!["consultar", "detalhar", "criar", "alterar"],
        ),
        (
            "Segurado",
            "segurados",
            vec!["consultar", "detalhar", "criar", "alterar"],
        ),
        (
            "Apolice",
            "apolices",
            vec!["consultar", "detalhar", "criar", "alterar"],
        ),
        (
            "Sinistro",
            "sinistros",
            vec!["consultar", "detalhar", "criar", "alterar"],
        ),
        (
            "OrdemColeta",
            "ordens-coleta",
            vec!["consultar", "detalhar", "criar", "alterar"],
        ),
        (
            "LaudoTriagem",
            "triagem",
            vec!["consultar", "detalhar", "criar"],
        ),
    ];

    for (nome, prefixo, ops) in &contexts {
        let ctx_opts = ContextOptions {
            nome: nome.to_string(),
            base_path: base.clone(),
            prefixo: prefixo.to_string(),
            namespace_base: ns.clone(),
            com_entity: true,
            com_autorizacoes: false,
            operacoes: ops.iter().map(|s| s.to_string()).collect(),
        };
        gen_context(&ctx_opts)?;
        println!();
    }

    // ── Sobrescreve os Eloquent Models com implementações reais ──────────────

    println!("  {}", style("[ 2/5 ] Sobrescrevendo Eloquent Models com campos reais...").cyan().bold());
    println!();

    let models: Vec<(&str, String, &str)> = vec![
        (
            "Seguradora/Infra/Persistence/Models/Seguradora.php",
            tmpl::model_seguradora(ns),
            "Seguradora",
        ),
        (
            "Transportadora/Infra/Persistence/Models/Transportadora.php",
            tmpl::model_transportadora(ns),
            "Transportadora",
        ),
        (
            "Segurado/Infra/Persistence/Models/Segurado.php",
            tmpl::model_segurado(ns),
            "Segurado",
        ),
        (
            "Apolice/Infra/Persistence/Models/Apolice.php",
            tmpl::model_apolice(ns),
            "Apolice",
        ),
        (
            "Sinistro/Infra/Persistence/Models/Sinistro.php",
            tmpl::model_sinistro(ns),
            "Sinistro",
        ),
        (
            "OrdemColeta/Infra/Persistence/Models/OrdemColeta.php",
            tmpl::model_ordem_coleta(ns),
            "OrdemColeta",
        ),
        (
            "LaudoTriagem/Infra/Persistence/Models/LaudoTriagem.php",
            tmpl::model_laudo_triagem(ns),
            "LaudoTriagem",
        ),
    ];

    for (rel_path, content, label) in &models {
        let path = PathBuf::from(base).join(rel_path);
        write_file(&path, content)?;
        println!(
            "  {} model {}",
            style("overwrite").yellow(),
            style(label).white().bold()
        );
    }

    // ── Sub-entidades extras (sem context próprio) ────────────────────────────

    let extras: Vec<(&str, String, &str)> = vec![
        (
            "Sinistro/Infra/Persistence/Models/ItemSinistrado.php",
            tmpl::model_item_sinistrado(ns),
            "ItemSinistrado (sub-entity de Sinistro)",
        ),
        (
            "OrdemColeta/Infra/Persistence/Models/MovimentacaoLogistica.php",
            tmpl::model_movimentacao_logistica(ns),
            "MovimentacaoLogistica (sub-entity de OrdemColeta)",
        ),
        (
            "OrdemColeta/Infra/Persistence/Models/RecebimentoCd.php",
            tmpl::model_recebimento_cd(ns),
            "RecebimentoCd (sub-entity de OrdemColeta)",
        ),
    ];

    for (rel_path, content, label) in &extras {
        let path = PathBuf::from(base).join(rel_path);
        write_file(&path, content)?;
        println!(
            "  {} {}",
            style("criado").green(),
            style(label).dim()
        );
    }

    println!();

    // ── Migrations ───────────────────────────────────────────────────────────

    println!("  {}", style("[ 3/5 ] Gerando Migrations...").cyan().bold());
    println!();

    let migration_base = PathBuf::from(&opts.migration_path);

    let migrations: Vec<(&str, &str)> = vec![
        ("2025_01_01_000001_create_seguradoras_table.php",          tmpl::migration_seguradoras()),
        ("2025_01_01_000002_create_transportadoras_table.php",      tmpl::migration_transportadoras()),
        ("2025_01_01_000003_create_segurados_table.php",            tmpl::migration_segurados()),
        ("2025_01_01_000004_create_apolices_table.php",             tmpl::migration_apolices()),
        ("2025_01_01_000005_create_sinistros_table.php",            tmpl::migration_sinistros()),
        ("2025_01_01_000006_create_itens_sinistrados_table.php",    tmpl::migration_itens_sinistrados()),
        ("2025_01_01_000007_create_ordens_coleta_table.php",        tmpl::migration_ordens_coleta()),
        ("2025_01_01_000008_create_movimentacoes_logisticas_table.php", tmpl::migration_movimentacoes_logisticas()),
        ("2025_01_01_000009_create_recebimentos_cd_table.php",      tmpl::migration_recebimentos_cd()),
        ("2025_01_01_000010_create_laudos_triagem_table.php",       tmpl::migration_laudos_triagem()),
    ];

    for (filename, content) in &migrations {
        let path = migration_base.join(filename);
        write_file(&path, content)?;
        println!("  {} {}", style("migration").green(), style(filename).dim());
    }

    println!();

    // ── Manager JSON ─────────────────────────────────────────────────────────

    println!("  {}", style("[ 4/5 ] Gerando Manager JSON...").cyan().bold());
    println!();

    let manager_content = tmpl::manager_json(&opts.erp_id, &opts.company_name, &opts.warehouse_id);
    let manager_path = PathBuf::from(base).parent()
        .unwrap_or(Path::new("."))
        .join("logistica_reversa_manager.json");
    write_file(&manager_path, &manager_content)?;
    println!("  {} {}", style("criado").green(), style(manager_path.display()).dim());

    // ── [ 5/5 ] Infra Docker ───────────────────────────────────────────────

    docker::generator::scaffold(&opts.project_root, "logistica-reversa")?;

    // ── Resumo
    println!();
    println!("  {} Contexts DDD criados:", style("7").yellow().bold());
    println!("    Seguradora · Transportadora · Segurado · Apolice");
    println!("    Sinistro · OrdemColeta · LaudoTriagem (Triagem)");
    println!();
    println!("  {} Sub-entidades (dentro dos contexts):", style("3").yellow().bold());
    println!("    ItemSinistrado · MovimentacaoLogistica · RecebimentoCd");
    println!();
    println!("  {} Migrations (ordem de FK):", style("10").yellow().bold());
    println!("    seguradoras → transportadoras → segurados → apolices");
    println!("    → sinistros → itens_sinistrados → ordens_coleta");
    println!("    → movimentacoes_logisticas → recebimentos_cd → laudos_triagem");
    println!();
    println!(
        "  {} Registre os providers em {}:",
        style("IMPORTANTE:").yellow().bold(),
        style("bootstrap/providers.php").white()
    );
    for (nome, _, _) in &contexts {
        println!(
            "    {}\\{}\\Infra\\Providers\\{}ServiceProvider::class",
            style(ns).dim(),
            style(nome).white(),
            style(nome).white()
        );
    }
    println!();

    Ok(())
}

fn write_file(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)?;
    Ok(())
}

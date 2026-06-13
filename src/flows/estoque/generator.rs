use crate::flows::context::generator::{generate as gen_context, ContextOptions};
use crate::flows::docker;
use super::templates as tmpl;
use anyhow::Result;
use console::style;
use std::fs;
use std::path::{Path, PathBuf};

pub struct EstoqueOptions {
    pub project_root: String,
    pub base_path: String,
    pub namespace_base: String,
    pub migration_path: String,
    pub tenant_id: String,
    pub valuation_method: String,
}

pub fn generate(opts: &EstoqueOptions) -> Result<()> {
    let ns = &opts.namespace_base;
    let base = &opts.base_path;

    println!();
    println!("  {}", style("[ 1/6 ] Gerando DDD Contexts...").cyan().bold());
    println!();

    // ── 6 Bounded Contexts ───────────────────────────────────────────────────
    // MovimentacaoEstoque: NUNCA gera alterar/deletar — Kardex e imutavel!
    let contexts: Vec<(&str, &str, Vec<&str>)> = vec![
        (
            "Armazem",
            "armazens",
            vec!["consultar", "detalhar", "criar", "alterar"],
        ),
        (
            "Fornecedor",
            "fornecedores",
            vec!["consultar", "detalhar", "criar", "alterar"],
        ),
        (
            "Produto",
            "produtos",
            vec!["consultar", "detalhar", "criar", "alterar"],
        ),
        (
            "PedidoCompra",
            "pedidos-compra",
            vec!["consultar", "detalhar", "criar", "alterar"],
        ),
        (
            "MovimentacaoEstoque",
            "movimentacoes-estoque",
            // GET + POST apenas — Kardex é imutável, sem PUT/DELETE
            vec!["consultar", "detalhar", "criar"],
        ),
        (
            "Inventario",
            "inventarios",
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

    // ── Sobrescreve Eloquent Models com campos reais ─────────────────────────

    println!("  {}", style("[ 2/6 ] Sobrescrevendo Eloquent Models com campos reais...").cyan().bold());
    println!();

    let models: Vec<(&str, String, &str)> = vec![
        (
            "Armazem/Infra/Persistence/Models/Armazem.php",
            tmpl::model_armazem(ns),
            "Armazem",
        ),
        (
            "Fornecedor/Infra/Persistence/Models/Fornecedor.php",
            tmpl::model_fornecedor(ns),
            "Fornecedor",
        ),
        (
            "Produto/Infra/Persistence/Models/Produto.php",
            tmpl::model_produto(ns),
            "Produto",
        ),
        (
            "PedidoCompra/Infra/Persistence/Models/PedidoCompra.php",
            tmpl::model_pedido_compra(ns),
            "PedidoCompra",
        ),
        (
            "MovimentacaoEstoque/Infra/Persistence/Models/MovimentacaoEstoque.php",
            tmpl::model_movimentacao_estoque(ns),
            "MovimentacaoEstoque",
        ),
        (
            "Inventario/Infra/Persistence/Models/Inventario.php",
            tmpl::model_inventario(ns),
            "Inventario",
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

    // ── Sub-entidades extras ─────────────────────────────────────────────────

    let extras: Vec<(&str, String, &str)> = vec![
        (
            "Armazem/Infra/Persistence/Models/PosicaoEstoque.php",
            tmpl::model_posicao_estoque(ns),
            "PosicaoEstoque (sub-entity de Armazem)",
        ),
        (
            "Produto/Infra/Persistence/Models/Lote.php",
            tmpl::model_lote(ns),
            "Lote (sub-entity de Produto)",
        ),
        (
            "PedidoCompra/Infra/Persistence/Models/ItemPedidoCompra.php",
            tmpl::model_item_pedido_compra(ns),
            "ItemPedidoCompra (sub-entity de PedidoCompra)",
        ),
        (
            "Inventario/Infra/Persistence/Models/ContagemInventario.php",
            tmpl::model_contagem_inventario(ns),
            "ContagemInventario (sub-entity de Inventario)",
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

    // ── UseCases e Errors especiais ──────────────────────────────────────────

    println!("  {}", style("[ 3/6 ] Gerando UseCases e Errors especiais (Kardex + Inventario)...").cyan().bold());
    println!();

    // MovimentacaoEstoque — UseCase Kardex
    write_file(
        &PathBuf::from(base).join("MovimentacaoEstoque/Application/UseCases/RegistrarMovimentacaoUseCase.php"),
        &tmpl::usecase_registrar_movimentacao(ns),
    )?;
    println!("  {} RegistrarMovimentacaoUseCase (Kardex com saldo_apos_movimento)", style("criado").green());

    write_file(
        &PathBuf::from(base).join("MovimentacaoEstoque/Application/DTOs/Inputs/RegistrarMovimentacaoInput.php"),
        &tmpl::input_registrar_movimentacao(ns),
    )?;
    println!("  {} RegistrarMovimentacaoInput", style("criado").green());

    write_file(
        &PathBuf::from(base).join("MovimentacaoEstoque/Application/Errors/EstoqueInsuficienteError.php"),
        &tmpl::error_estoque_insuficiente(ns),
    )?;
    write_file(
        &PathBuf::from(base).join("MovimentacaoEstoque/Application/Errors/EstoqueNegativoNaoPermitidoError.php"),
        &tmpl::error_estoque_negativo(ns),
    )?;
    println!("  {} EstoqueInsuficienteError + EstoqueNegativoNaoPermitidoError", style("criado").green());

    // Inventario — UseCase fechar + ajuste Kardex automatico
    write_file(
        &PathBuf::from(base).join("Inventario/Application/UseCases/FecharInventarioUseCase.php"),
        &tmpl::usecase_fechar_inventario(ns),
    )?;
    println!("  {} FecharInventarioUseCase (gera ajustes Kardex automaticamente)", style("criado").green());

    write_file(
        &PathBuf::from(base).join("Inventario/Application/DTOs/Inputs/FecharInventarioInput.php"),
        &tmpl::input_fechar_inventario(ns),
    )?;
    write_file(
        &PathBuf::from(base).join("Inventario/Application/Errors/InventarioJaFinalizadoError.php"),
        &tmpl::error_inventario_ja_finalizado(ns),
    )?;
    println!("  {} FecharInventarioInput + InventarioJaFinalizadoError", style("criado").green());

    println!();

    // ── Migrations ───────────────────────────────────────────────────────────

    println!("  {}", style("[ 4/6 ] Gerando Migrations (ordem de FK)...").cyan().bold());
    println!();

    let migration_base = PathBuf::from(&opts.migration_path);

    let migrations: Vec<(&str, &str)> = vec![
        ("2025_02_01_000001_create_armazens_table.php",            tmpl::migration_armazens()),
        ("2025_02_01_000002_create_posicoes_estoque_table.php",    tmpl::migration_posicoes_estoque()),
        ("2025_02_01_000003_create_fornecedores_table.php",        tmpl::migration_fornecedores()),
        ("2025_02_01_000004_create_produtos_table.php",            tmpl::migration_produtos()),
        ("2025_02_01_000005_create_lotes_table.php",               tmpl::migration_lotes()),
        ("2025_02_01_000006_create_pedidos_compra_table.php",      tmpl::migration_pedidos_compra()),
        ("2025_02_01_000007_create_itens_pedido_compra_table.php", tmpl::migration_itens_pedido_compra()),
        ("2025_02_01_000008_create_movimentacoes_estoque_table.php", tmpl::migration_movimentacoes_estoque()),
        ("2025_02_01_000009_create_inventarios_table.php",         tmpl::migration_inventarios()),
        ("2025_02_01_000010_create_contagens_inventario_table.php", tmpl::migration_contagens_inventario()),
    ];

    for (filename, content) in &migrations {
        let path = migration_base.join(filename);
        write_file(&path, content)?;
        println!("  {} {}", style("migration").green(), style(filename).dim());
    }

    println!();

    // ── Manager JSON ─────────────────────────────────────────────────────────

    println!("  {}", style("[ 5/6 ] Gerando Manager JSON...").cyan().bold());
    println!();

    let manager_content = tmpl::manager_json(&opts.tenant_id, &opts.valuation_method);
    let manager_path = PathBuf::from(base)
        .parent()
        .unwrap_or(Path::new("."))
        .join("estoque_manager.json");
    write_file(&manager_path, &manager_content)?;
    println!("  {} {}", style("criado").green(), style(manager_path.display()).dim());

    // ── [ 6/6 ] Infra Docker ───────────────────────────────────────────────

    docker::generator::scaffold(&opts.project_root, "estoque")?;

    // ── Resumo
    println!();
    println!("  {} Contexts DDD:", style("6").yellow().bold());
    println!("    Armazem · Fornecedor · Produto · PedidoCompra");
    println!("    MovimentacaoEstoque (Kardex, imutavel) · Inventario");
    println!();
    println!("  {} Sub-entidades:", style("4").yellow().bold());
    println!("    PosicaoEstoque · Lote · ItemPedidoCompra · ContagemInventario");
    println!();
    println!("  {} UseCases especiais:", style("2").yellow().bold());
    println!("    RegistrarMovimentacaoUseCase — calcula saldo_apos_movimento (Kardex)");
    println!("    FecharInventarioUseCase      — gera ajustes automaticos no Kardex");
    println!();
    println!("  {} Migrations (FK order):", style("10").yellow().bold());
    println!("    armazens → posicoes_estoque → fornecedores → produtos → lotes");
    println!("    → pedidos_compra → itens_pedido_compra");
    println!("    → movimentacoes_estoque → inventarios → contagens_inventario");
    println!();
    println!(
        "  {} Registre os 6 providers em {}:",
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

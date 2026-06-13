use super::templates as tmpl;
use anyhow::Result;
use console::style;
use std::fs;
use std::path::{Path, PathBuf};

pub struct ContextOptions {
    pub nome: String,
    pub base_path: String,
    pub prefixo: String,
    pub namespace_base: String,
    pub com_entity: bool,
    pub com_autorizacoes: bool,
    pub operacoes: Vec<String>,
}

pub fn generate(opts: &ContextOptions) -> Result<()> {
    let name = &opts.nome;
    let ns = &opts.namespace_base;
    let base = PathBuf::from(&opts.base_path).join(name);
    let prefix = &opts.prefixo;
    let ops = &opts.operacoes;

    // ── Application/DTOs/Inputs ──────────────────────────────────────────────
    if ops.contains(&"criar".to_string()) {
        write_file(
            &base.join(format!("Application/DTOs/Inputs/Criar{name}Input.php")),
            &tmpl::input_criar(ns, name),
        )?;
    }
    if ops.contains(&"alterar".to_string()) {
        write_file(
            &base.join(format!("Application/DTOs/Inputs/Alterar{name}Input.php")),
            &tmpl::input_alterar(ns, name),
        )?;
    }

    // ── Application/DTOs/Outputs ─────────────────────────────────────────────
    if ops.contains(&"consultar".to_string()) {
        write_file(
            &base.join(format!(
                "Application/DTOs/Outputs/Consultar{name}sOutput.php"
            )),
            &tmpl::output_consultar(ns, name),
        )?;
    }
    if ops.contains(&"detalhar".to_string()) {
        write_file(
            &base.join(format!("Application/DTOs/Outputs/Detalhar{name}Output.php")),
            &tmpl::output_detalhar(ns, name),
        )?;
    }

    // ── Application/Errors ───────────────────────────────────────────────────
    write_file(
        &base.join(format!("Application/Errors/{name}NaoEncontradoError.php")),
        &tmpl::error_nao_encontrado(ns, name),
    )?;

    // ── Application/Exceptions ───────────────────────────────────────────────
    write_file(
        &base.join(format!("Application/Exceptions/{name}Exception.php")),
        &tmpl::exception(ns, name),
    )?;

    // ── Application/Queries ──────────────────────────────────────────────────
    if ops.contains(&"consultar".to_string()) {
        write_file(
            &base.join(format!(
                "Application/Queries/Consultar{name}sQuery.php"
            )),
            &tmpl::query_consultar(ns, name),
        )?;
    }
    if ops.contains(&"detalhar".to_string()) {
        write_file(
            &base.join(format!("Application/Queries/Detalhar{name}Query.php")),
            &tmpl::query_detalhar(ns, name),
        )?;
    }

    // ── Application/UseCases ─────────────────────────────────────────────────
    if ops.contains(&"criar".to_string()) {
        write_file(
            &base.join(format!("Application/UseCases/Criar{name}UseCase.php")),
            &tmpl::usecase_criar(ns, name),
        )?;
    }
    if ops.contains(&"alterar".to_string()) {
        write_file(
            &base.join(format!("Application/UseCases/Alterar{name}UseCase.php")),
            &tmpl::usecase_alterar(ns, name),
        )?;
    }
    if ops.contains(&"deletar".to_string()) {
        write_file(
            &base.join(format!("Application/UseCases/Deletar{name}UseCase.php")),
            &tmpl::usecase_deletar(ns, name),
        )?;
    }

    // ── Domain/Entities ──────────────────────────────────────────────────────
    if opts.com_entity {
        write_file(
            &base.join(format!("Domain/Entities/{name}Entity.php")),
            &tmpl::entity(ns, name),
        )?;
    }

    // ── Domain/Autorizacoes ──────────────────────────────────────────────────
    if opts.com_autorizacoes {
        write_file(
            &base.join(format!("Domain/Autorizacoes/{name}Autorizacoes.php")),
            &tmpl::autorizacoes(ns, name),
        )?;
    }

    // ── Infra/Persistence ────────────────────────────────────────────────────
    write_file(
        &base.join(format!("Infra/Persistence/Models/{name}.php")),
        &tmpl::eloquent_model(ns, name),
    )?;
    write_file(
        &base.join(format!(
            "Infra/Persistence/Repositories/{name}Repository.php"
        )),
        &tmpl::repository(ns, name),
    )?;

    // ── Infra/Presentation/Http/Controllers ──────────────────────────────────
    write_file(
        &base.join(format!(
            "Infra/Presentation/Http/Controllers/{name}Controller.php"
        )),
        &tmpl::controller(ns, name, ops),
    )?;

    // ── Infra/Presentation/Http/Requests ─────────────────────────────────────
    if ops.contains(&"criar".to_string()) {
        write_file(
            &base.join(format!(
                "Infra/Presentation/Http/Requests/Criar{name}Request.php"
            )),
            &tmpl::request_criar(ns, name),
        )?;
    }
    if ops.contains(&"alterar".to_string()) {
        write_file(
            &base.join(format!(
                "Infra/Presentation/Http/Requests/Alterar{name}Request.php"
            )),
            &tmpl::request_alterar(ns, name),
        )?;
    }

    // ── Infra/Presentation/Routes ────────────────────────────────────────────
    write_file(
        &base.join("Infra/Presentation/Routes/api.php"),
        &tmpl::routes(ns, name, ops),
    )?;

    // ── Infra/Providers ──────────────────────────────────────────────────────
    write_file(
        &base.join(format!("Infra/Providers/{name}ServiceProvider.php")),
        &tmpl::service_provider(ns, name, prefix),
    )?;

    // ── Summary ──────────────────────────────────────────────────────────────
    println!(
        "  {} {}",
        style("Estrutura criada em:").dim(),
        style(base.display()).white()
    );
    println!();
    println!(
        "  {} Registre o provider em bootstrap/providers.php:",
        style("IMPORTANTE:").yellow().bold()
    );
    println!(
        "    {}\\{}\\Infra\\Providers\\{}ServiceProvider::class",
        style(ns).dim(),
        style(name).white(),
        style(name).white()
    );

    Ok(())
}

fn write_file(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)?;
    println!(
        "  {} {}",
        style("criado").green(),
        style(path.display()).dim()
    );
    Ok(())
}

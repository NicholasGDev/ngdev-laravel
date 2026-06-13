pub mod generator;
pub mod templates;

use anyhow::Result;
use console::style;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect};
use heck::{ToKebabCase, ToPascalCase};

pub fn run(theme: &ColorfulTheme) -> Result<()> {
    println!("{}", style("  [ Criar Context DDD ]").yellow().bold());
    println!();

    let project_root: String = Input::with_theme(theme)
        .with_prompt("  Caminho absoluto do projeto Laravel (ex: /home/user/meu-projeto)")
        .interact_text()?;
    let root = project_root.trim_end_matches('/');

    let nome_raw: String = Input::with_theme(theme)
        .with_prompt("  Nome do Context (ex: Cliente, Produto, Pedido)")
        .interact_text()?;

    let nome = nome_raw.trim().to_pascal_case();
    let prefixo_default = nome.to_kebab_case() + "s";

    let prefixo: String = Input::with_theme(theme)
        .with_prompt("  Prefixo da rota (kebab-case)")
        .default(prefixo_default)
        .interact_text()?;

    let base_path: String = Input::with_theme(theme)
        .with_prompt("  Diretorio base dos Contexts (relativo ao projeto)")
        .default("app/Contexts".to_string())
        .interact_text()?;

    let namespace_base: String = Input::with_theme(theme)
        .with_prompt("  Namespace base PHP")
        .default("App\\Contexts".to_string())
        .interact_text()?;

    let com_entity = Confirm::with_theme(theme)
        .with_prompt("  Gerar Domain Entity?")
        .default(true)
        .interact()?;

    let com_autorizacoes = Confirm::with_theme(theme)
        .with_prompt("  Gerar Autorizacoes?")
        .default(false)
        .interact()?;

    let ops_labels = &[
        "consultar (GET /consultar)",
        "detalhar  (GET /detalhar/{id})",
        "criar     (POST /criar)",
        "alterar   (PUT /alterar/{id})",
        "deletar   (DELETE /deletar/{id})",
    ];
    let ops_keys = &["consultar", "detalhar", "criar", "alterar", "deletar"];
    // Todos selecionados por padrão — CRUD completo
    let defaults = &[true, true, true, true, true];

    let selecionados = MultiSelect::with_theme(theme)
        .with_prompt("  Quais operacoes gerar? [Espaco=selecionar, Enter=confirmar]")
        .items(ops_labels)
        .defaults(defaults)
        .interact()?;

    let operacoes: Vec<String> = selecionados.iter().map(|&i| ops_keys[i].to_string()).collect();

    let base_path = if base_path.starts_with('/') { base_path }
                    else { format!("{}/{}", root, base_path) };

    println!();
    println!(
        "  {} '{}'...",
        style("Gerando Context").yellow(),
        style(&nome).white().bold()
    );
    println!();

    generator::generate(&generator::ContextOptions {
        nome: nome.clone(),
        base_path,
        prefixo,
        namespace_base,
        com_entity,
        com_autorizacoes,
        operacoes,
    })?;

    println!();
    println!(
        "  {} Context '{}' gerado com sucesso!",
        style("✔").green().bold(),
        style(&nome).white().bold()
    );
    println!(
        "  {} Registre o provider em {}",
        style("LEMBRETE:").yellow().bold(),
        style("bootstrap/providers.php").white()
    );
    println!();

    Ok(())
}

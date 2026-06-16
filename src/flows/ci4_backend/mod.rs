pub mod generator;
pub mod templates;

use anyhow::Result;
use console::style;
use dialoguer::{theme::ColorfulTheme, Input};
use generator::{Ci4Options, generate};

pub fn run(theme: &ColorfulTheme) -> Result<()> {
    println!("{}", style("  [ Gerar Backend CodeIgniter 4 ]").yellow().bold());
    println!();
    println!(
        "  {}",
        style("SQLite · JWT · Export JSON + CSV · FTP-ready · auto-migrate via URL").dim()
    );
    println!();

    let app_name: String = Input::with_theme(theme)
        .with_prompt("  Nome do projeto (slug, ex: meu-backend)")
        .default("meu-backend".to_string())
        .interact_text()?;

    let company_name: String = Input::with_theme(theme)
        .with_prompt("  Nome da empresa / app (exibido nas respostas)")
        .default(app_name.replace('-', " ").replace('_', " "))
        .interact_text()?;

    let base_url: String = Input::with_theme(theme)
        .with_prompt("  URL base da API  (ex: https://api.meusite.com.br/)")
        .default("http://localhost:8080/".to_string())
        .interact_text()?;

    let cors_origin: String = Input::with_theme(theme)
        .with_prompt("  URL da landing page para CORS  (ex: https://meusite.com.br  ou  *)")
        .default("*".to_string())
        .interact_text()?;

    let output_dir: String = Input::with_theme(theme)
        .with_prompt("  Caminho de saída (absoluto)")
        .interact_text()?;

    let opts = Ci4Options {
        app_name,
        company_name,
        base_url,
        cors_origin,
        output_dir,
    };

    generate(&opts)?;

    println!();
    println!(
        "  {} Backend CI4 gerado em '{}'",
        style("✔").green().bold(),
        style(&opts.output_dir).white().bold()
    );
    println!();
    println!("  {} Próximos passos:", style("LEMBRETE:").yellow().bold());
    println!("    cd {}", opts.output_dir);
    println!("    composer install");
    println!("    cp env .env           # edite JWT_SECRET, SETUP_TOKEN, app.baseURL");
    println!("    # Upload via FTP (tudo, inclusive vendor/)");
    println!("    # Acesse no browser para criar as tabelas:");
    println!("    GET /setup/migrate?token=SEU_SETUP_TOKEN");
    println!();
    println!("  {} Credenciais admin padrão (criadas pelo migrate):", style("INFO:").cyan().bold());
    println!("    email : admin@caronte.local");
    println!("    senha : caronte@2026");
    println!("    POST  /auth/login  →  retorna JWT");
    println!();
    println!("  {} Endpoints protegidos (header: Authorization: Bearer <token>):", style("API:").cyan().bold());
    println!("    GET /auth/me");
    println!("    GET /export/tables");
    println!("    GET /export/json/{{tabela}}");
    println!("    GET /export/csv/{{tabela}}");
    println!("  {} Endpoint público:", style("API:").cyan().bold());
    println!("    POST /contact    (captura lead da landing page)");
    println!();

    Ok(())
}

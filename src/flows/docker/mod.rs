pub mod generator;
pub mod templates;

use anyhow::Result;
use console::style;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};

pub fn run(theme: &ColorfulTheme) -> Result<()> {
    println!("{}", style("  [ Gerar Infra Docker ]").yellow().bold());
    println!();
    println!("  {}", style("Gera Dockerfile.dev + Dockerfile.prod + docker-compose (dev/prod)").dim());
    println!("  {}", style("+ Nginx + PHP-FPM + Xdebug + Supervisor + Makefile + .env.example").dim());
    println!();

    let app_name: String = Input::with_theme(theme)
        .with_prompt("  Nome da aplicacao")
        .default("MyApp".to_string())
        .interact_text()?;

    let output_path: String = Input::with_theme(theme)
        .with_prompt("  Diretorio de saida (raiz do projeto Laravel)")
        .default(".".to_string())
        .interact_text()?;

    let server_name: String = Input::with_theme(theme)
        .with_prompt("  server_name do Nginx (ex: localhost ou app.local)")
        .default("localhost".to_string())
        .interact_text()?;

    let php_versions = vec!["8.3 (latest stable)", "8.2", "8.1"];
    let php_idx = Select::with_theme(theme)
        .with_prompt("  Versao do PHP")
        .items(&php_versions)
        .default(0)
        .interact()?;
    let php_version = match php_idx {
        0 => "8.3",
        1 => "8.2",
        _ => "8.1",
    };

    let node_versions = vec!["22 (LTS current)", "20 (LTS)"];
    let node_idx = Select::with_theme(theme)
        .with_prompt("  Versao do Node.js")
        .items(&node_versions)
        .default(0)
        .interact()?;
    let node_version: u8 = if node_idx == 0 { 22 } else { 20 };

    let db_labels = &[
        "MySQL 8.4",
        "MariaDB 11.4",
        "PostgreSQL 17",
        "SQL Server 2022",
        "SQLite (sem container, apenas extensao)",
    ];
    let db_keys = &["mysql", "mariadb", "pgsql", "sqlserver", "sqlite"];
    let db_defaults = &[true, false, false, false, false];

    let db_selecionados = MultiSelect::with_theme(theme)
        .with_prompt("  Bancos de dados [Espaco=selecionar]")
        .items(db_labels)
        .defaults(db_defaults)
        .interact()?;

    let databases: Vec<String> = db_selecionados
        .iter()
        .map(|&i| db_keys[i].to_string())
        .collect();

    let with_redis = Confirm::with_theme(theme)
        .with_prompt("  Incluir Redis (cache + queue + session)?")
        .default(true)
        .interact()?;

    let with_mailpit = Confirm::with_theme(theme)
        .with_prompt("  Incluir Mailpit (SMTP local para dev)?")
        .default(true)
        .interact()?;

    println!();
    println!("  {}", style("Gerando infra...").yellow().bold());
    println!();

    generator::generate(&generator::DockerOptions {
        app_name,
        php_version: php_version.to_string(),
        node_version,
        output_path,
        server_name,
        databases,
        with_redis,
        with_mailpit,
    })?;

    Ok(())
}

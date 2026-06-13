use super::templates as tmpl;
use anyhow::Result;
use console::style;
use std::fs;
use std::path::{Path, PathBuf};

pub struct DockerOptions {
    pub app_name: String,
    pub php_version: String,
    pub node_version: u8,
    pub output_path: String,
    pub server_name: String,
    pub databases: Vec<String>,
    pub with_redis: bool,
    pub with_mailpit: bool,
}

pub fn generate(opts: &DockerOptions) -> Result<()> {
    let base = PathBuf::from(&opts.output_path);

    println!();
    println!("  {}", style("[ 1/7 ] Gerando Dockerfiles (dev + prod)...").cyan().bold());
    println!();

    // ── Dockerfiles ───────────────────────────────────────────────────────────
    write(&base.join("Dockerfile.dev"),
        &tmpl::dockerfile_dev(&opts.php_version, opts.node_version, &opts.databases))?;
    ok("Dockerfile.dev");

    write(&base.join("Dockerfile.prod"),
        &tmpl::dockerfile_prod(&opts.php_version, &opts.databases))?;
    ok("Dockerfile.prod");

    // ── docker-compose ────────────────────────────────────────────────────────
    println!();
    println!("  {}", style("[ 2/7 ] Gerando docker-compose (dev + prod)...").cyan().bold());
    println!();

    write(
        &base.join("docker-compose.dev.yml"),
        &tmpl::compose_dev(&opts.app_name, &opts.databases, opts.with_redis, opts.with_mailpit),
    )?;
    ok("docker-compose.dev.yml");

    write(
        &base.join("docker-compose.prod.yml"),
        &tmpl::compose_prod(&opts.app_name, &opts.databases, opts.with_redis),
    )?;
    ok("docker-compose.prod.yml");

    // ── Nginx ─────────────────────────────────────────────────────────────────
    println!();
    println!("  {}", style("[ 3/7 ] Gerando configs Nginx...").cyan().bold());
    println!();

    write(&base.join("docker/nginx/nginx.conf"), tmpl::nginx_conf())?;
    ok("docker/nginx/nginx.conf");

    write(
        &base.join("docker/nginx/default.conf"),
        &tmpl::nginx_default_conf(&opts.server_name),
    )?;
    ok("docker/nginx/default.conf");

    // ── PHP ini ───────────────────────────────────────────────────────────────
    println!();
    println!("  {}", style("[ 4/7 ] Gerando configs PHP (dev + prod + xdebug)...").cyan().bold());
    println!();

    write(&base.join("docker/php/php-dev.ini"), tmpl::php_ini_dev())?;
    ok("docker/php/php-dev.ini");

    write(&base.join("docker/php/php-prod.ini"), tmpl::php_ini_prod())?;
    ok("docker/php/php-prod.ini");

    write(&base.join("docker/php/xdebug.ini"), tmpl::xdebug_ini())?;
    ok("docker/php/xdebug.ini");

    write(&base.join("docker/php/www.conf"), tmpl::php_fpm_www_conf())?;
    ok("docker/php/www.conf  (PHP-FPM pool)");

    // ── MySQL conf ────────────────────────────────────────────────────────────
    if opts.databases.iter().any(|d| d == "mysql" || d == "mariadb") {
        println!();
        println!("  {}", style("[ 5/7 ] Gerando config MySQL...").cyan().bold());
        println!();

        write(&base.join("docker/mysql/my.cnf"), tmpl::mysql_conf())?;
        ok("docker/mysql/my.cnf");
    } else {
        println!();
        println!("  {} (MySQL nao selecionado, pulando)", style("[ 5/7 ]").dim());
        println!();
    }

    // ── Supervisor ────────────────────────────────────────────────────────────
    println!();
    println!("  {}", style("[ 6/7 ] Gerando Supervisor (prod)...").cyan().bold());
    println!();

    write(&base.join("docker/supervisor/supervisord.prod.conf"), tmpl::supervisord_prod())?;
    ok("docker/supervisor/supervisord.prod.conf");

    // ── .dockerignore + .env.example + Makefile ───────────────────────────────
    println!();
    println!("  {}", style("[ 7/7 ] Gerando .dockerignore, .env.example, Makefile...").cyan().bold());
    println!();

    write(&base.join(".dockerignore"), tmpl::dockerignore())?;
    ok(".dockerignore");

    write(
        &base.join(".env.example"),
        &tmpl::env_example(&opts.app_name, &opts.databases, opts.with_redis),
    )?;
    ok(".env.example");

    write(&base.join("Makefile"), &tmpl::makefile(&opts.app_name))?;
    ok("Makefile");

    // ── Resumo ───────────────────────────────────────────────────────────────
    println!();
    println!("  {}", style("══════════════════════════════════════════════════════").dim());
    println!("  {}", style("  Infra Docker gerada com sucesso!").green().bold());
    println!("  {}", style("══════════════════════════════════════════════════════").dim());
    println!();
    println!("  {} PHP {} | Node {}", style("Stack:").yellow().bold(), style(&opts.php_version).white(), style(opts.node_version).white());
    println!();

    let db_labels: Vec<&str> = opts.databases.iter().map(|d| match d.as_str() {
        "mysql"     => "MySQL 8.4",
        "mariadb"   => "MariaDB 11.4",
        "pgsql"     => "PostgreSQL 17",
        "sqlserver" => "SQL Server 2022",
        "sqlite"    => "SQLite 3",
        _           => d.as_str(),
    }).collect();
    println!("  {} {}", style("Bancos:").yellow().bold(), db_labels.join(" · "));
    if opts.with_redis   { println!("  {} Redis 7.4", style("+").green()); }
    if opts.with_mailpit { println!("  {} Mailpit (SMTP/UI)", style("+").green()); }
    println!();
    println!("  {} Proximos passos:", style("PROXIMOS PASSOS:").yellow().bold());
    println!("    cp .env.example .env");
    println!("    php artisan key:generate");
    println!("    make build   # Constroi as imagens");
    println!("    make up      # Sobe DEV");
    println!("    make artisan cmd=migrate");
    println!();
    println!("  {} Em producao:", style("PROD:").yellow().bold());
    println!("    make prod-build");
    println!("    make prod-up");
    println!();

    Ok(())
}

fn write(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)?;
    Ok(())
}

fn ok(label: &str) {
    println!("  {} {}", style("criado").green(), style(label).dim());
}

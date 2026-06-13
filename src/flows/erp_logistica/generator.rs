use super::templates as tmpl;
use crate::flows::deps;
use anyhow::Result;
use console::style;
use std::fs;
use std::path::{Path, PathBuf};

pub fn generate(project_root: &str) -> Result<()> {
    let root = PathBuf::from(project_root);

    // ── 1. Laravel base ───────────────────────────────────────────────────────
    deps::copy_laravel_base(project_root)?;

    // ── 2. Migrations ─────────────────────────────────────────────────────────
    println!("  {}", style("[ 1/4 ] Gerando Migrations...").cyan().bold());
    let mig_dir = root.join("database/migrations");
    fs::create_dir_all(&mig_dir)?;
    for (filename, content) in tmpl::migrations() {
        let path = mig_dir.join(filename);
        write_file(&path, content)?;
    }
    println!();

    // ── 3. Models ─────────────────────────────────────────────────────────────
    println!("  {}", style("[ 2/4 ] Gerando Models (App\\Models)...").cyan().bold());
    let model_dir = root.join("app/Models");
    fs::create_dir_all(&model_dir)?;
    for (filename, content) in tmpl::models() {
        let path = model_dir.join(filename);
        write_file(&path, content)?;
    }
    println!();

    // ── 4. Services ───────────────────────────────────────────────────────────
    println!("  {}", style("[ 3/4 ] Gerando Services (App\\Services)...").cyan().bold());
    let svc_dir = root.join("app/Services");
    fs::create_dir_all(&svc_dir)?;
    for (filename, content) in tmpl::services() {
        let path = svc_dir.join(filename);
        write_file(&path, content)?;
    }
    println!();

    // ── 5. Controllers ────────────────────────────────────────────────────────
    println!("  {}", style("[ 4/4 ] Gerando Controllers (App\\Http\\Controllers\\Logistica)...").cyan().bold());
    let ctrl_dir = root.join("app/Http/Controllers");
    for (filename, content) in tmpl::controllers() {
        let path = ctrl_dir.join(filename);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        write_file(&path, content)?;
    }
    println!();

    // ── 6. Routes ─────────────────────────────────────────────────────────────
    let routes_dir = root.join("routes");
    fs::create_dir_all(&routes_dir)?;
    write_file(&routes_dir.join("api_erp_logistica.php"), tmpl::ROUTES)?;

    // ── Resumo ────────────────────────────────────────────────────────────────
    println!("  {}", style("══════════════════════════════════════════════════════").dim());
    println!("  {}", style("  ERP Logística Reversa MVCS gerado com sucesso!").green().bold());
    println!("  {}", style("══════════════════════════════════════════════════════").dim());
    println!();
    println!("  {} Próximos passos:", style("PRÓXIMOS PASSOS:").yellow().bold());
    println!("    cd {}", project_root);
    println!("    composer install");
    println!("    cp .env.example .env && php artisan key:generate");
    println!("    php artisan migrate");
    println!();
    println!("  {} Registre as rotas em routes/api.php:", style("ROTAS:").yellow().bold());
    println!("    require __DIR__.'/api_erp_logistica.php';");
    println!();
    println!("  {} Módulos gerados:", style("MÓDULOS:").cyan().bold());
    println!("    Seguradora · Transportadora · Segurado · Apolice");
    println!("    Sinistro · ItemSinistrado · OrdemColeta");
    println!("    MovimentacaoLogistica · RecebimentoCd · LaudoTriagem");
    println!();

    Ok(())
}

fn write_file(path: &Path, content: &str) -> Result<()> {
    fs::write(path, content)?;
    println!("  {} {}", style("criado").green(), style(path.display()).dim());
    Ok(())
}

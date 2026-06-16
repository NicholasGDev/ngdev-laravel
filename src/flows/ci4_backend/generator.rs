use super::templates as t;
use anyhow::Result;
use console::style;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use zip::ZipArchive;

pub struct Ci4Options {
    pub app_name:    String,
    pub company_name: String,
    pub base_url:    String,
    pub cors_origin: String,
    pub output_dir:  String,
}

fn write(path: &PathBuf, content: &str) -> Result<()> {
    if let Some(p) = path.parent() { fs::create_dir_all(p)?; }
    fs::write(path, content)?;
    Ok(())
}

pub fn generate(opts: &Ci4Options) -> Result<()> {
    let base = PathBuf::from(&opts.output_dir);

    // ── 1. Extrair CaronteCI4-base.zip ────────────────────────────────────────
    println!(
        "  {} Extraindo base CI4 para '{}'...",
        style("→").cyan(),
        style(&opts.output_dir).white().bold()
    );

    let cursor  = std::io::Cursor::new(t::CARONTE_CI4_BASE_ZIP);
    let mut archive = ZipArchive::new(cursor)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let raw      = file.name().to_string();
        let name     = raw.trim_start_matches("./");
        let out      = base.join(name);

        if file.is_dir() {
            fs::create_dir_all(&out)?;
        } else {
            if let Some(p) = out.parent() { fs::create_dir_all(p)?; }
            let mut buf = Vec::with_capacity(file.size() as usize);
            file.read_to_end(&mut buf)?;
            fs::write(&out, &buf)?;
        }
    }

    // ── 2. Sobrescrever arquivos customizados ──────────────────────────────────
    let cfg  = base.join("app").join("Config");
    let ctrl = base.join("app").join("Controllers");
    let filt = base.join("app").join("Filters");
    let lib  = base.join("app").join("Libraries");
    let mdl  = base.join("app").join("Models");
    let mig  = base.join("app").join("Database").join("Migrations");

    write(&cfg.join("App.php"),      &t::config_app(&opts.base_url))?;
    write(&cfg.join("Database.php"), t::CONFIG_DATABASE)?;
    write(&cfg.join("Filters.php"),  t::CONFIG_FILTERS)?;
    write(&cfg.join("Routes.php"),   t::CONFIG_ROUTES)?;

    write(&lib.join("Jwt.php"), t::LIB_JWT)?;

    write(&filt.join("JwtFilter.php"),  t::FILTER_JWT)?;
    write(&filt.join("CorsFilter.php"), t::FILTER_CORS)?;

    write(&ctrl.join("BaseController.php"), t::CTRL_BASE)?;
    write(&ctrl.join("Auth.php"),           t::CTRL_AUTH)?;
    write(&ctrl.join("Contact.php"),        &t::ctrl_contact(&opts.company_name))?;
    write(&ctrl.join("Export.php"),         t::CTRL_EXPORT)?;
    write(&ctrl.join("Setup.php"),          t::CTRL_SETUP)?;

    write(&mdl.join("UserModel.php"), t::MODEL_USER)?;
    write(&mdl.join("LeadModel.php"), t::MODEL_LEAD)?;

    write(&mig.join("2026-01-01-000001_CreateUsersTable.php"), t::MIG_USERS)?;
    write(&mig.join("2026-01-01-000002_CreateLeadsTable.php"), t::MIG_LEADS)?;

    // ── 3. .env ───────────────────────────────────────────────────────────────
    write(&base.join("env"), &t::env_template(&opts.base_url, &opts.cors_origin))?;

    // ── 4. Garantir writable/database/ gravável ───────────────────────────────
    fs::create_dir_all(base.join("writable").join("database"))?;

    println!("  {} Base CI4 extraída e configurada.", style("✔").green().bold());
    Ok(())
}

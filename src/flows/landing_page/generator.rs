use anyhow::Result;
use console::style;
use std::fs;
use std::path::PathBuf;

pub struct LandingPageOptions {
    pub product_name: String,
    pub tagline: String,
    pub company_name: String,
    pub theme: String,
    pub sections: Vec<String>,
    /// Diretório de saída (será criado se não existir).
    /// Estrutura gerada:
    ///   <output_dir>/index.html
    ///   <output_dir>/sections/navbar.html
    ///   <output_dir>/sections/hero.html
    ///   <output_dir>/sections/<secao>.html  (para cada seção selecionada)
    ///   <output_dir>/sections/footer.html
    pub output_dir: String,
}

/// Grava um arquivo criando o diretório pai se necessário.
fn write(path: &PathBuf, content: &str) -> Result<()> {
    if let Some(p) = path.parent() {
        fs::create_dir_all(p)?;
    }
    fs::write(path, content)?;
    Ok(())
}

pub fn generate(opts: &LandingPageOptions) -> Result<()> {
    let base = PathBuf::from(&opts.output_dir);
    let sections_dir = base.join("sections");
    fs::create_dir_all(&sections_dir)?;

    // ── Seções fixas ──────────────────────────────────────────────────────────
    let navbar_html  = super::templates::section_navbar(&opts.product_name);
    let hero_html    = super::templates::section_hero(&opts.product_name, &opts.tagline);
    let footer_html  = super::templates::section_footer(&opts.company_name);

    write(&sections_dir.join("navbar.html"), &navbar_html)?;
    write(&sections_dir.join("hero.html"),   &hero_html)?;
    write(&sections_dir.join("footer.html"), &footer_html)?;

    // ── Seções opcionais ──────────────────────────────────────────────────────
    let order: &[(&str, &str)] = &[
        ("logos",          "logos.html"),
        ("features_grid",  "features_grid.html"),
        ("features_tabs",  "features_tabs.html"),
        ("stats",          "stats.html"),
        ("testimonials",   "testimonials.html"),
        ("pricing",        "pricing.html"),
        ("faq",            "faq.html"),
        ("cta_bottom",     "cta_bottom.html"),
    ];

    let mut body_parts: Vec<String> = vec![navbar_html.clone(), hero_html.clone()];

    for (key, filename) in order {
        if opts.sections.contains(&key.to_string()) {
            let fragment = match *key {
                "logos"         => super::templates::section_logos().to_string(),
                "features_grid" => super::templates::section_features_grid().to_string(),
                "features_tabs" => super::templates::section_features_tabs().to_string(),
                "stats"         => super::templates::section_stats().to_string(),
                "testimonials"  => super::templates::section_testimonials().to_string(),
                "pricing"       => super::templates::section_pricing().to_string(),
                "faq"           => super::templates::section_faq().to_string(),
                "cta_bottom"    => super::templates::section_cta_bottom(&opts.product_name),
                _               => String::new(),
            };
            write(&sections_dir.join(filename), &fragment)?;
            body_parts.push(fragment);
        }
    }

    body_parts.push(footer_html);

    // ── index.html (página montada) ───────────────────────────────────────────
    let body = body_parts.join("\n");
    let index_html = super::templates::html_shell(&opts.product_name, &opts.theme, &body);
    let index_path = base.join("index.html");
    write(&index_path, &index_html)?;

    // ── Relatório ─────────────────────────────────────────────────────────────
    println!();
    println!(
        "  {} Landing page gerada em: {}",
        style("✔").green().bold(),
        style(opts.output_dir.as_str()).white().bold()
    );
    println!(
        "  {} {}",
        style("→").dim(),
        style("index.html").cyan()
    );
    println!(
        "  {} {}",
        style("→").dim(),
        style("sections/navbar.html · hero.html · footer.html").cyan()
    );

    let active: Vec<&str> = order
        .iter()
        .filter(|(k, _)| opts.sections.contains(&k.to_string()))
        .map(|(_, f)| *f)
        .collect();

    if !active.is_empty() {
        println!(
            "  {} sections/{}",
            style("→").dim(),
            style(active.join(" · ")).cyan()
        );
    }

    if let Ok(abs) = fs::canonicalize(&index_path) {
        println!(
            "\n  {} Abra: {}",
            style("↗").yellow(),
            style(format!("file://{}", abs.display())).underlined()
        );
    }

    Ok(())
}

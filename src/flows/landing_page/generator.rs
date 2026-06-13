use super::templates;
use super::templates_saas;
use anyhow::Result;
use console::style;
use std::fs;
use std::path::PathBuf;

pub struct LandingPageOptions {
    pub product_name: String,
    pub tagline: String,
    pub company_name: String,
    pub theme: String,
    /// "generic" (padrão DaisyUI) ou "saas" (layout Contabilizei-style)
    pub layout: String,
    pub sections: Vec<String>,
    /// Diretório de saída — gera:
    ///   <output_dir>/index.html
    ///   <output_dir>/sections/navbar.html  hero.html  footer.html  <secao>.html
    pub output_dir: String,
}

fn write_file(path: &PathBuf, content: &str) -> Result<()> {
    if let Some(p) = path.parent() {
        fs::create_dir_all(p)?;
    }
    fs::write(path, content)?;
    Ok(())
}

pub fn generate(opts: &LandingPageOptions) -> Result<()> {
    if opts.layout == "saas" {
        generate_saas(opts)
    } else {
        generate_generic(opts)
    }
}

// ── Layout genérico (DaisyUI / produto) ──────────────────────────────────────
fn generate_generic(opts: &LandingPageOptions) -> Result<()> {
    let base = PathBuf::from(&opts.output_dir);
    let sections_dir = base.join("sections");
    fs::create_dir_all(&sections_dir)?;

    let navbar_html = templates::section_navbar(&opts.product_name);
    let hero_html   = templates::section_hero(&opts.product_name, &opts.tagline);
    let footer_html = templates::section_footer(&opts.company_name);

    write_file(&sections_dir.join("navbar").join("index.html"), &navbar_html)?;
    write_file(&sections_dir.join("hero").join("index.html"),   &hero_html)?;
    write_file(&sections_dir.join("footer").join("index.html"), &footer_html)?;

    let order: &[(&str, &str)] = &[
        ("logos",         "logos"),
        ("features_grid", "features_grid"),
        ("features_tabs", "features_tabs"),
        ("stats",         "stats"),
        ("testimonials",  "testimonials"),
        ("pricing",       "pricing"),
        ("faq",           "faq"),
        ("cta_bottom",    "cta_bottom"),
    ];

    let mut body_parts = vec![navbar_html.clone(), hero_html.clone()];

    for (key, dir_name) in order {
        if opts.sections.contains(&key.to_string()) {
            let fragment = match *key {
                "logos"         => templates::section_logos().to_string(),
                "features_grid" => templates::section_features_grid().to_string(),
                "features_tabs" => templates::section_features_tabs().to_string(),
                "stats"         => templates::section_stats().to_string(),
                "testimonials"  => templates::section_testimonials().to_string(),
                "pricing"       => templates::section_pricing().to_string(),
                "faq"           => templates::section_faq().to_string(),
                "cta_bottom"    => templates::section_cta_bottom(&opts.product_name),
                _               => String::new(),
            };
            write_file(&sections_dir.join(dir_name).join("index.html"), &fragment)?;
            body_parts.push(fragment);
        }
    }

    body_parts.push(footer_html);
    let body = body_parts.join("\n");
    let index_html = templates::html_shell(&opts.product_name, &opts.theme, &body);
    let index_path = base.join("index.html");
    write_file(&index_path, &index_html)?;

    print_report(opts, order, &index_path)
}

// ── Layout SaaS / Serviço (Contabilizei-style) ───────────────────────────────
fn generate_saas(opts: &LandingPageOptions) -> Result<()> {
    let base = PathBuf::from(&opts.output_dir);
    let sections_dir = base.join("sections");
    fs::create_dir_all(&sections_dir)?;

    let navbar_html = templates_saas::section_navbar_saas(&opts.product_name);
    let hero_html   = templates_saas::section_hero_saas(&opts.product_name, &opts.tagline);
    let footer_html = templates_saas::section_footer_saas(&opts.company_name);

    write_file(&sections_dir.join("navbar").join("index.html"), &navbar_html)?;
    write_file(&sections_dir.join("hero").join("index.html"),   &hero_html)?;
    write_file(&sections_dir.join("footer").join("index.html"), &footer_html)?;

    let order: &[(&str, &str)] = &[
        ("social_proof",       "social_proof"),
        ("comparison_table",   "comparison_table"),
        ("journey_selector",   "journey_selector"),
        ("benefits_slider",    "benefits_slider"),
        ("content_grid",       "content_grid"),
        ("testimonials_photo", "testimonials_photo"),
        ("faq",                "faq"),
    ];

    let mut body_parts = vec![navbar_html.clone(), hero_html.clone()];

    for (key, dir_name) in order {
        if opts.sections.contains(&key.to_string()) {
            let fragment = match *key {
                "social_proof"       => templates_saas::section_social_proof().to_string(),
                "comparison_table"   => templates_saas::section_comparison_table(&opts.product_name),
                "journey_selector"   => templates_saas::section_journey_selector().to_string(),
                "benefits_slider"    => templates_saas::section_benefits_slider(&opts.product_name),
                "content_grid"       => templates_saas::section_content_grid().to_string(),
                "testimonials_photo" => templates_saas::section_testimonials_photo().to_string(),
                "faq"                => templates_saas::section_faq_saas().to_string(),
                _                    => String::new(),
            };
            write_file(&sections_dir.join(dir_name).join("index.html"), &fragment)?;
            body_parts.push(fragment);
        }
    }

    body_parts.push(footer_html);
    let body = body_parts.join("\n");
    let index_html = templates_saas::html_shell_saas(&opts.product_name, &body);
    let index_path = base.join("index.html");
    write_file(&index_path, &index_html)?;

    print_report(opts, order, &index_path)
}

// ── Relatório ─────────────────────────────────────────────────────────────────
fn print_report(opts: &LandingPageOptions, order: &[(&str, &str)], index_path: &PathBuf) -> Result<()> {
    println!();
    println!(
        "  {} Landing page [{}] gerada em: {}",
        style("✔").green().bold(),
        opts.layout,
        style(opts.output_dir.as_str()).white().bold(),
    );
    println!("  {} {}", style("→").dim(), style("index.html").cyan());
    println!(
        "  {} {}",
        style("→").dim(),
        style("sections/navbar/index.html · sections/hero/index.html · sections/footer/index.html").cyan()
    );

    let active: Vec<&str> = order
        .iter()
        .filter(|(k, _)| opts.sections.contains(&k.to_string()))
        .map(|(_, d)| *d)
        .collect();

    if !active.is_empty() {
        for dir in &active {
            println!(
                "  {} sections/{}/index.html",
                style("→").dim(),
                style(dir).cyan()
            );
        }
    }

    if let Ok(abs) = fs::canonicalize(index_path) {
        println!(
            "\n  {} Abra: {}",
            style("↗").yellow(),
            style(format!("file://{}", abs.display())).underlined()
        );
    }
    Ok(())
}

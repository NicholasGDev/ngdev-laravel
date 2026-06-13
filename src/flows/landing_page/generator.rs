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
    pub output_file: String,
}

pub fn generate(opts: &LandingPageOptions) -> Result<()> {
    let mut body = String::new();

    // Sempre: navbar + hero
    body.push_str(&super::templates::section_navbar(&opts.product_name));
    body.push_str(&super::templates::section_hero(&opts.product_name, &opts.tagline));

    // Seções opcionais — respeitam a ordem de exibição da landing page
    let order = [
        "logos",
        "features_grid",
        "features_tabs",
        "stats",
        "testimonials",
        "pricing",
        "faq",
        "cta_bottom",
    ];

    for &key in &order {
        if opts.sections.contains(&key.to_string()) {
            let fragment = match key {
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
            body.push_str(&fragment);
        }
    }

    // Sempre: footer
    body.push_str(&super::templates::section_footer(&opts.company_name));

    let html = super::templates::html_shell(&opts.product_name, &opts.theme, &body);

    let path = PathBuf::from(&opts.output_file);
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    fs::write(&path, &html)?;

    let abs = fs::canonicalize(&path)?;
    println!(
        "\n  {} Landing page gerada: {}",
        style("✔").green().bold(),
        style(path.display().to_string()).white().bold()
    );
    println!(
        "  {} Abra no navegador: {}",
        style("→").dim(),
        style(format!("file://{}", abs.display())).cyan()
    );
    println!(
        "  {} Seções: navbar · hero{} · footer",
        style("✦").yellow(),
        if opts.sections.is_empty() {
            String::new()
        } else {
            format!(" · {}", opts.sections.join(" · "))
        }
    );

    Ok(())
}

pub mod generator;

use anyhow::Result;
use console::style;
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect, Select};

use crate::flows::ci4_backend::generator::{Ci4Options, generate as generate_ci4};
use crate::flows::landing_page::generator::LandingPageOptions;
use generator::generate_combined;

pub fn run(theme: &ColorfulTheme) -> Result<()> {
    println!("{}", style("  [ Landing Page + Backend CI4 — Fluxo Completo ]").yellow().bold());
    println!();
    println!(
        "  {}",
        style("Gera a landing page com formulário de contato já conectado ao backend CI4.").dim()
    );
    println!();

    // ─────────────────────────────────────────────────────────────────────────
    // LANDING PAGE — perguntas
    // ─────────────────────────────────────────────────────────────────────────
    let layout_idx = Select::with_theme(theme)
        .with_prompt("  Layout base")
        .items(&[
            "Generic   — DaisyUI · múltiplos temas · produto/SaaS clássico",
            "SaaS      — Layout Contabilizei-style · fundo branco · verde",
        ])
        .default(0)
        .interact()?;
    let layout = if layout_idx == 1 { "saas" } else { "generic" };

    println!();

    let product_name: String = Input::with_theme(theme)
        .with_prompt("  Nome do produto / marca")
        .interact_text()?;

    let tagline: String = Input::with_theme(theme)
        .with_prompt("  Tagline (subtítulo do hero)")
        .default("A solução completa para o seu negócio.".to_string())
        .interact_text()?;

    let company_name: String = Input::with_theme(theme)
        .with_prompt("  Nome da empresa (rodapé + copyright)")
        .default(product_name.clone())
        .interact_text()?;

    let (sections, selected_theme) = if layout == "saas" {
        let section_labels = [
            "Prova Social",
            "Tabela Comparativa",
            "Seletor de Jornada",
            "Slider de Benefícios",
            "Grid de Conteúdo",
            "Depoimentos com Foto",
            "FAQ",
        ];
        let section_keys = [
            "social_proof", "comparison_table", "journey_selector",
            "benefits_slider", "content_grid", "testimonials_photo", "faq",
        ];
        let sel = MultiSelect::with_theme(theme)
            .with_prompt("  Seções a incluir")
            .items(&section_labels)
            .defaults(&[true; 7])
            .interact()?;
        let secs = sel.iter().map(|&i| section_keys[i].to_string()).collect();
        (secs, "light".to_string())
    } else {
        let theme_keys = ["light","dark","corporate","lofi","business","cupcake","cyberpunk","forest","luxury","night"];
        let theme_idx = Select::with_theme(theme)
            .with_prompt("  Tema DaisyUI")
            .items(&[
                "light","dark","corporate","lofi","business",
                "cupcake","cyberpunk","forest","luxury","night",
            ])
            .default(0)
            .interact()?;
        let section_labels = [
            "Logos / Parceiros",
            "Grade de Features",
            "Features Detalhadas",
            "Estatísticas",
            "Depoimentos",
            "Tabela de Preços",
            "FAQ",
            "CTA Final",
        ];
        let section_keys = ["logos","features_grid","features_tabs","stats","testimonials","pricing","faq","cta_bottom"];
        let sel = MultiSelect::with_theme(theme)
            .with_prompt("  Seções a incluir")
            .items(&section_labels)
            .defaults(&[true; 8])
            .interact()?;
        let secs = sel.iter().map(|&i| section_keys[i].to_string()).collect();
        (secs, theme_keys[theme_idx].to_string())
    };

    let lp_output: String = Input::with_theme(theme)
        .with_prompt("  Caminho de saída da Landing Page (absoluto)")
        .interact_text()?;

    println!();
    println!("  {}", style("── Backend CI4 ──────────────────────────────────────────────────").dim());
    println!();

    // ─────────────────────────────────────────────────────────────────────────
    // BACKEND CI4 — perguntas
    // ─────────────────────────────────────────────────────────────────────────
    let app_name: String = Input::with_theme(theme)
        .with_prompt("  Slug do projeto backend (ex: meu-backend)")
        .default(
            product_name.to_lowercase().replace(' ', "-").replace('_', "-")
        )
        .interact_text()?;

    let base_url: String = Input::with_theme(theme)
        .with_prompt("  URL base da API CI4  (ex: https://api.meusite.com.br/)")
        .default("http://localhost:8080/".to_string())
        .interact_text()?;

    // CORS = a URL da landing page (usada no backend)
    let lp_url: String = Input::with_theme(theme)
        .with_prompt("  URL da Landing Page (para CORS, ex: https://meusite.com.br)")
        .default("*".to_string())
        .interact_text()?;

    let ci4_output: String = Input::with_theme(theme)
        .with_prompt("  Caminho de saída do Backend CI4 (absoluto)")
        .interact_text()?;

    // ─────────────────────────────────────────────────────────────────────────
    // Gerar ambos
    // ─────────────────────────────────────────────────────────────────────────
    let lp_opts = LandingPageOptions {
        product_name:  product_name.clone(),
        tagline:       tagline.clone(),
        company_name:  company_name.clone(),
        theme:         selected_theme,
        layout:        layout.to_string(),
        sections,
        output_dir:    lp_output.clone(),
        api_url:       base_url.trim_end_matches('/').to_string(),
    };

    let ci4_opts = Ci4Options {
        app_name,
        company_name,
        base_url:     base_url.clone(),
        cors_origin:  lp_url,
        output_dir:   ci4_output.clone(),
    };

    generate_combined(&lp_opts, &ci4_opts)?;
    generate_ci4(&ci4_opts)?;

    println!();
    println!("  {} Gerado com sucesso!", style("✔").green().bold());
    println!();
    println!("  {} Landing Page : {}", style("→").cyan(), style(&lp_output).white().bold());
    println!("  {} Backend CI4  : {}", style("→").cyan(), style(&ci4_output).white().bold());
    println!();
    println!("  {} Próximos passos:", style("LEMBRETE:").yellow().bold());
    println!("    # Backend:");
    println!("    cd {}", ci4_output);
    println!("    composer install");
    println!("    cp env .env   # ajuste JWT_SECRET, SETUP_TOKEN, app.baseURL");
    println!("    GET {base_url}setup/migrate?token=SEU_SETUP_TOKEN");
    println!();
    println!("    # Upload ambas as pastas via FTP");
    println!();

    Ok(())
}

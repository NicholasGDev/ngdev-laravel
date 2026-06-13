pub mod generator;
pub mod templates;
pub mod templates_saas;

use anyhow::Result;
use console::style;
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect, Select};

pub fn run(theme: &ColorfulTheme) -> Result<()> {
    println!("{}", style("  [ Gerar Landing Page ]").yellow().bold());
    println!();

    // ── Layout ────────────────────────────────────────────────────────────────
    let layout_labels = vec![
        "Generic   — DaisyUI · múltiplos temas · produto/SaaS clássico",
        "SaaS      — Layout Contabilizei-style · fundo branco · verde · serviços",
    ];
    let layout_idx = Select::with_theme(theme)
        .with_prompt("  Layout base")
        .items(&layout_labels)
        .default(0)
        .interact()?;
    let layout = if layout_idx == 1 { "saas" } else { "generic" };

    println!();
    println!("  {}", style("Arquivos: index.html + sections/*.html separados").dim());
    println!();

    let product_name: String = Input::with_theme(theme)
        .with_prompt("  Nome do produto / marca")
        .interact_text()?;

    let tagline: String = Input::with_theme(theme)
        .with_prompt("  Tagline (subtítulo do hero)")
        .default(
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
             Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
                .to_string(),
        )
        .interact_text()?;

    let company_name: String = Input::with_theme(theme)
        .with_prompt("  Nome da empresa (rodapé + copyright)")
        .default(product_name.clone())
        .interact_text()?;

    // ── Seções e tema dependem do layout ─────────────────────────────────────
    let (sections, selected_theme) = if layout == "saas" {
        let section_labels = [
            "Prova Social          — RA1000 + Google stars + contador de clientes",
            "Tabela Comparativa    — vs. média do mercado (preços/benefícios)",
            "Seletor de Jornada    — tabs PF / MEI / Trocar contador com imagem",
            "Slider de Benefícios  — cartões horizontais scroll de features",
            "Grid de Conteúdo      — 3 artigos / blog com imagem",
            "Depoimentos com Foto  — grid 3x2 com nome, cargo e avaliação",
            "FAQ                   — accordion clean (fundo branco)",
        ];
        let section_keys = [
            "social_proof", "comparison_table", "journey_selector",
            "benefits_slider", "content_grid", "testimonials_photo", "faq",
        ];
        let defaults = [true, true, true, true, true, true, true];
        let selected_indices = MultiSelect::with_theme(theme)
            .with_prompt("  Seções a incluir [Espaço=toggle · Enter=confirmar]")
            .items(&section_labels)
            .defaults(&defaults)
            .interact()?;
        let secs = selected_indices.iter().map(|&i| section_keys[i].to_string()).collect();
        (secs, "light".to_string()) // saas always light
    } else {
        let theme_labels = vec![
            "light      — Claro padrão",
            "dark       — Escuro elegante",
            "corporate  — Corporativo azul",
            "lofi       — Minimalista P&B",
            "business   — Business escuro",
            "cupcake    — Soft / pastel",
            "cyberpunk  — Cyberpunk neon",
            "forest     — Verde natural",
            "luxury     — Preto premium",
            "night      — Azul noturno",
        ];
        let theme_keys = ["light","dark","corporate","lofi","business","cupcake","cyberpunk","forest","luxury","night"];
        let theme_idx = Select::with_theme(theme)
            .with_prompt("  Tema DaisyUI")
            .items(&theme_labels)
            .default(0)
            .interact()?;

        let section_labels = [
            "Logos / Empresas parceiras    — marquee animado",
            "Grade de Features             — 4 cards com ícones SVG",
            "Features Detalhadas           — 3 tabs + mock browser screenshot",
            "Estatísticas em destaque      — 4 números grandes (fundo colorido)",
            "Depoimentos                   — 3 cards de clientes",
            "Tabela de Preços              — Free / Pro / Enterprise",
            "FAQ                           — accordion DaisyUI (5 perguntas)",
            "CTA Final                     — seção de conversão com gradiente",
        ];
        let section_keys = ["logos","features_grid","features_tabs","stats","testimonials","pricing","faq","cta_bottom"];
        let defaults = [true, true, true, true, true, true, true, true];
        let selected_indices = MultiSelect::with_theme(theme)
            .with_prompt("  Seções a incluir [Espaço=toggle · Enter=confirmar]")
            .items(&section_labels)
            .defaults(&defaults)
            .interact()?;
        let secs = selected_indices.iter().map(|&i| section_keys[i].to_string()).collect();
        (secs, theme_keys[theme_idx].to_string())
    };

    let output_dir: String = Input::with_theme(theme)
        .with_prompt("  Diretório de saída")
        .default("landing".to_string())
        .interact_text()?;

    println!();
    println!("  {}", style("Montando landing page...").yellow().bold());
    println!();

    generator::generate(&generator::LandingPageOptions {
        product_name,
        tagline,
        company_name,
        theme: selected_theme,
        layout: layout.to_string(),
        sections,
        output_dir,
    })?;

    Ok(())
}

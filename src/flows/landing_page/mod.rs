pub mod generator;
pub mod templates;

use anyhow::Result;
use console::style;
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect, Select};

pub fn run(theme: &ColorfulTheme) -> Result<()> {
    println!("{}", style("  [ Gerar Landing Page ]").yellow().bold());
    println!();
    println!("  {}", style("HTML único · Tailwind CSS CDN · DaisyUI · Lorem Ipsum · pronto para editar").dim());
    println!("  {}", style("Seções modulares: navbar, hero, logos, features, stats, preços, FAQ, footer").dim());
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
    let theme_keys = [
        "light", "dark", "corporate", "lofi", "business",
        "cupcake", "cyberpunk", "forest", "luxury", "night",
    ];

    let theme_idx = Select::with_theme(theme)
        .with_prompt("  Tema DaisyUI")
        .items(&theme_labels)
        .default(0)
        .interact()?;

    let selected_theme = theme_keys[theme_idx];

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
    let section_keys = [
        "logos", "features_grid", "features_tabs", "stats",
        "testimonials", "pricing", "faq", "cta_bottom",
    ];
    let defaults = [true, true, true, true, true, true, true, true];

    let selected_indices = MultiSelect::with_theme(theme)
        .with_prompt("  Seções a incluir [Espaço=toggle · Enter=confirmar]")
        .items(&section_labels)
        .defaults(&defaults)
        .interact()?;

    let sections: Vec<String> = selected_indices
        .iter()
        .map(|&i| section_keys[i].to_string())
        .collect();

    let output_dir: String = Input::with_theme(theme)
        .with_prompt("  Diretorio de saida")
        .default("landing".to_string())
        .interact_text()?;

    println!();
    println!("  {}", style("Montando landing page...").yellow().bold());
    println!();

    generator::generate(&generator::LandingPageOptions {
        product_name,
        tagline,
        company_name,
        theme: selected_theme.to_string(),
        sections,
        output_dir,
    })?;

    Ok(())
}

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
        style("Landing page gerada em public/ do CI4 — uma pasta, um deploy.").dim()
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

    // slug do produto: "Meu Produto" → "meu-produto"
    let slug = product_name
        .to_lowercase()
        .replace(' ', "-")
        .replace('_', "-");

    println!();
    println!(
        "  {}",
        style("Exemplo de caminho de saída:").dim()
    );
    println!(
        "  {}  {}",
        style("Linux  →").cyan(),
        style(format!("/home/usuario/projetos/{slug}")).white()
    );
    println!(
        "  {}  {}",
        style("Windows→").cyan(),
        style(format!("C:\\projetos\\{slug}")).white()
    );
    println!();

    let project_root: String = Input::with_theme(theme)
        .with_prompt("  Caminho da pasta do projeto (absoluto)")
        .interact_text()?;

    let base_url: String = Input::with_theme(theme)
        .with_prompt("  URL base da API CI4  (ex: https://api.meusite.com.br/)")
        .default("http://localhost:8080/".to_string())
        .interact_text()?;

    // CORS: mesma origem ou subdomínio diferente. Se tudo estiver no mesmo
    // domínio (landing page em public/ + API no mesmo host) usar "*" está OK.
    let lp_url: String = Input::with_theme(theme)
        .with_prompt("  Origem permitida no CORS  (mesmo domínio = *  |  outro = https://meusite.com.br)")
        .default("*".to_string())
        .interact_text()?;

    // CI4 = raiz do projeto; Landing Page vai em <raiz>/public/ (web root do CI4)
    let ci4_output = project_root.clone();
    let lp_output  = format!("{}/public", project_root.trim_end_matches('/'));

    // ─────────────────────────────────────────────────────────────────────────
    // Gerar: primeiro CI4 (cria a estrutura), depois a landing em public/
    // ─────────────────────────────────────────────────────────────────────────
    let ci4_opts = Ci4Options {
        app_name:     slug.clone(),
        company_name: company_name.clone(),
        base_url:     base_url.clone(),
        cors_origin:  lp_url,
        output_dir:   ci4_output.clone(),
    };

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

    // 1) extrai CI4 e escreve configs customizadas
    generate_ci4(&ci4_opts)?;
    // 2) gera landing page dentro de public/ (index.html + sections/)
    generate_combined(&lp_opts, &ci4_opts)?;

    println!();
    println!("  {} Gerado com sucesso!", style("✔").green().bold());
    println!();
    println!("  {} Estrutura criada:", style("→").cyan());
    println!("    {}/", style(&project_root).white().bold());
    println!("    ├── app/                ← CI4: controllers, models, migrations...");
    println!("    ├── public/             ← web root (Apache/Nginx aponta aqui)");
    println!("    │   ├── index.html      ← landing page (servida diretamente)");
    println!("    │   ├── index.php       ← CI4 front controller (rotas da API)");
    println!("    │   └── sections/       ← fragmentos HTML da landing");
    println!("    ├── writable/           ← banco SQLite + logs");
    println!("    └── env                 ← copie para .env e configure");
    println!();
    println!("  {} Próximos passos:", style("LEMBRETE:").yellow().bold());
    println!("    cd {}", project_root);
    println!("    cp env .env   # edite JWT_SECRET, SETUP_TOKEN, app.baseURL");
    println!("    # Aponte o VirtualHost para:  {}/public", project_root);
    println!("    # Acesse no browser para criar as tabelas:");
    println!("    GET {base_url}setup/migrate?token=SEU_SETUP_TOKEN");
    println!();

    Ok(())
}

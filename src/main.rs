mod cli;
mod generators;
mod templates;

use anyhow::Result;
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};
use heck::{ToKebabCase, ToPascalCase};

const LOGO: &str = r#"
##########################################################################
##########################################################################
##########################################################################
#################################################################****#####
###############################################################*=:..:+*###
##############################################################*-.    .+###
##############################################################*:      -###
##############################################################*-     .+###
##############################################################*: ...:=####
#####################*****###**+++**##############**+++***###*: -***######
####################=.....+*=:.    .-+*########*+-.     .-+*+:.-##########
##############*=-*##=     -.         .=#######+:          .:..=#+#########
############*=. :*##+                  =#####=.       ..    .=#-.*########
##########*-.   :*##+       :=++-.     .*###=     .-+***+:..+*- .*########
########+-.   .=*###+      =#%%%%*:    .+##*.    .+#%%%%%#=+*-  .*########
######+:    :=#%####+     :#####%%=     =##+.    =#%###%%%#*:   .*########
#####+.. .:+#%######+     -#######+     =##=    .*#########+    .*########
#####+...:*#########+.   .=#######+     =##=.   .*#########=    .*########
#####*:....-+#######+.  ..-#######+.    =##+.   .+#########:.   .*########
#######+:....:=*####+.....=#######+.....=###: ..=*+*#####*-......*########
######%%#*-....:=###+.....=#######+.....=##%+..+*-.:-=+=-.......:*########
########%%#*=...:###+.....=#######+.....=##%%+**-...............:*########
##########%%%#=::###+.....=#######+.....=#####*:..........--....:*########
###########%%%%#*###+.....=#######+.....=####*:.-+-:::::=*#=....:*########
##############%%%###*=----+#######*--=--*###+:.=#%######%##:....:#%%###%##
####################%%%%%%%######%%%%%#####+::=#*###%#####=:..:.=#%%%%%%%%
#####################%%%%%########%#######=.:+*-:-=+****+-:.::::#%%%%%%%%%
#####################################*=-:-::+#-::::::::::::::::*%%%%%%%%%%
####################################+::::::-#*-::::::::::::::-*%%%#%#%####
####################################-:::::::*##*+=-:::::::-+*%%%%%#%%%####
####################################=:::::::*%%%%%########%%%%%%%%#%%%####
####################################*-:::::+#%%%%%%%%%%%%%%%%%%%%%#%%%####
###################################%%#*+=+*#%%##%%%%%%%%%%%%%%%%%%%%%%####
###################################%%%%%%%%#######%%%%%%%%%#%%%%%%#%%%=###
####################################%%%%%%%%###%%%%%%###%%%#%%%%%%##%%+###
#####################################%%##%#####%##%%#%%#%%###%%##%#%%%%%%%
#########################################%####%%%%%%%%%%%%%%%%%%%%%%%%%%%%
########################################%%##########%%%%%%%%%%%#%%#%%%%%%%
######################################%%#######%#%#%%#%%%%%%%%%%%%%%%%%%%%
#######################################%%%%#%#%%%%%%%%%####%%%%%%%%%%%%%%%
#############%%%###################%##%%%%##%#%%%%%%%%%###%%#%%%%%%%%%#%%%



                    ╔══════════════════════════════════════╗
                    ║                                      ║
                    ║           Ng Development             ║
                    ║              Laravel                 ║
                    ║          context creator             ║
                    ║                                      ║
                    ╚══════════════════════════════════════╝
"#;

fn main() -> Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;

    println!("{}", style(LOGO).cyan().bold());
    println!("{}", style("  Bem-vindo ao gerador de codigo Laravel DDD").dim());
    println!("{}", style("  ─────────────────────────────────────────────").dim());
    println!();

    let theme = ColorfulTheme::default();

    loop {
        let opcoes = vec![
            "Criar Context (DDD Completo)",
            "Scaffold Logistica Reversa de Sinistros (DDD Completo)",
            "Scaffold ERP Estoque (DDD Completo)",
            "Gerar Infra Docker (dev + prod, multi-DB)",
            "Criar Model",
            "Criar Controller",
            "Criar Migration",
            "Scaffold PDV Completo",
            "Sair",
        ];

        let selecao = Select::with_theme(&theme)
            .with_prompt("O que deseja gerar?")
            .items(&opcoes)
            .default(0)
            .interact()?;

        println!();

        match selecao {
            0 => wizard_context(&theme)?,
            1 => wizard_logistica_reversa(&theme)?,
            2 => wizard_estoque(&theme)?,
            3 => wizard_docker(&theme)?,
            4 => wizard_model(&theme)?,
            5 => wizard_controller(&theme)?,
            6 => wizard_migration(&theme)?,
            7 => wizard_pdv(&theme)?,
            8 => {
                println!("{}", style("  Ate logo!").cyan().bold());
                break;
            }
            _ => unreachable!(),
        }

        println!();
        println!("{}", style("  ─────────────────────────────────────────────").dim());
        println!();
    }

    Ok(())
}

// ─── Context Wizard ──────────────────────────────────────────────────────────

fn wizard_context(theme: &ColorfulTheme) -> Result<()> {
    println!("{}", style("  [ Criar Context DDD ]").yellow().bold());
    println!();

    let nome_raw: String = Input::with_theme(theme)
        .with_prompt("  Nome do Context (ex: Cliente, Produto, Pedido)")
        .interact_text()?;

    let nome = nome_raw.trim().to_pascal_case();
    let prefixo_default = nome.to_kebab_case() + "s";

    let prefixo: String = Input::with_theme(theme)
        .with_prompt("  Prefixo da rota (kebab-case)")
        .default(prefixo_default)
        .interact_text()?;

    let base_path: String = Input::with_theme(theme)
        .with_prompt("  Diretorio base dos Contexts")
        .default("back/app/Contexts".to_string())
        .interact_text()?;

    let namespace_base: String = Input::with_theme(theme)
        .with_prompt("  Namespace base PHP")
        .default("App\\Contexts".to_string())
        .interact_text()?;

    let com_entity = Confirm::with_theme(theme)
        .with_prompt("  Gerar Domain Entity?")
        .default(true)
        .interact()?;

    let com_autorizacoes = Confirm::with_theme(theme)
        .with_prompt("  Gerar Autorizacoes?")
        .default(false)
        .interact()?;

    let ops_labels = &[
        "consultar (GET /consultar)",
        "detalhar  (GET /detalhar/{id})",
        "criar     (POST /criar)",
        "alterar   (PUT /alterar/{id})",
        "deletar   (DELETE /deletar/{id})",
    ];
    let ops_keys = &["consultar", "detalhar", "criar", "alterar", "deletar"];
    let defaults = &[true, true, true, true, false];

    let selecionados = MultiSelect::with_theme(theme)
        .with_prompt("  Quais operacoes gerar? [Espaco=selecionar, Enter=confirmar]")
        .items(ops_labels)
        .defaults(defaults)
        .interact()?;

    let operacoes: Vec<String> = selecionados.iter().map(|&i| ops_keys[i].to_string()).collect();

    println!();
    println!(
        "  {} '{}'...",
        style("Gerando Context").yellow(),
        style(&nome).white().bold()
    );
    println!();

    let opts = generators::context::ContextOptions {
        nome: nome.clone(),
        base_path,
        prefixo,
        namespace_base,
        com_entity,
        com_autorizacoes,
        operacoes,
    };

    generators::context::generate(&opts)?;

    println!();
    println!(
        "  {} Context '{}' gerado com sucesso!",
        style("").green().bold(),
        style(&nome).white().bold()
    );

    Ok(())
}

// ─── Model Wizard ─────────────────────────────────────────────────────────────

fn wizard_model(theme: &ColorfulTheme) -> Result<()> {
    println!("{}", style("  [ Criar Model ]").yellow().bold());
    println!();

    let nome: String = Input::with_theme(theme)
        .with_prompt("  Nome do Model (ex: User, BlogPost)")
        .interact_text()?;

    let com_migration = Confirm::with_theme(theme)
        .with_prompt("  Gerar Migration tambem?")
        .default(false)
        .interact()?;

    let com_controller = Confirm::with_theme(theme)
        .with_prompt("  Gerar Controller tambem?")
        .default(false)
        .interact()?;

    generators::model::generate(&cli::ModelArgs {
        name: nome,
        migration: com_migration,
        controller: com_controller,
    })?;

    Ok(())
}

// ─── Controller Wizard ────────────────────────────────────────────────────────

fn wizard_controller(theme: &ColorfulTheme) -> Result<()> {
    println!("{}", style("  [ Criar Controller ]").yellow().bold());
    println!();

    let nome: String = Input::with_theme(theme)
        .with_prompt("  Nome do Controller (ex: UserController)")
        .interact_text()?;

    let resource = Confirm::with_theme(theme)
        .with_prompt("  Gerar controller de resource?")
        .default(true)
        .interact()?;

    let model_nome: String = Input::with_theme(theme)
        .with_prompt("  Vincular a um Model? (deixe em branco para pular)")
        .allow_empty(true)
        .interact_text()?;

    generators::controller::generate(&cli::ControllerArgs {
        name: nome,
        resource,
        model: if model_nome.trim().is_empty() { None } else { Some(model_nome) },
    })?;

    Ok(())
}

// ─── Migration Wizard ─────────────────────────────────────────────────────────

fn wizard_migration(theme: &ColorfulTheme) -> Result<()> {
    println!("{}", style("  [ Criar Migration ]").yellow().bold());
    println!();

    let nome: String = Input::with_theme(theme)
        .with_prompt("  Nome da Migration (ex: create_users_table)")
        .interact_text()?;

    let tabela: String = Input::with_theme(theme)
        .with_prompt("  Nome da tabela (deixe em branco para detectar automaticamente)")
        .allow_empty(true)
        .interact_text()?;

    generators::migration::generate(&cli::MigrationArgs {
        name: nome,
        table: if tabela.trim().is_empty() { None } else { Some(tabela) },
    })?;

    Ok(())
}

// ─── PDV Wizard ───────────────────────────────────────────────────────────────

fn wizard_pdv(theme: &ColorfulTheme) -> Result<()> {
    println!("{}", style("  [ Scaffold PDV ]").yellow().bold());
    println!();

    let opcoes = vec![
        "Gerar Migrations e Models",
        "Gerar somente Migrations",
        "Gerar somente Models",
    ];

    let selecao = Select::with_theme(theme)
        .with_prompt("  O que gerar para o PDV?")
        .items(&opcoes)
        .default(0)
        .interact()?;

    generators::pdv::generate(&cli::PdvArgs {
        migrations_only: selecao == 1,
        models_only: selecao == 2,
    })?;

    Ok(())
}

// ─── Logistica Reversa Wizard ─────────────────────────────────────────────────

fn wizard_logistica_reversa(theme: &ColorfulTheme) -> Result<()> {
    println!("{}", style("  [ Scaffold Logistica Reversa de Sinistros ]").yellow().bold());
    println!();
    println!("  {}", style("Gera 7 Contexts DDD + 10 Migrations + 10 Eloquent Models + Manager JSON").dim());
    println!("  {}", style("Entidades: Seguradora · Transportadora · Segurado · Apolice").dim());
    println!("  {}", style("           Sinistro · OrdemColeta · LaudoTriagem").dim());
    println!();

    let base_path: String = Input::with_theme(theme)
        .with_prompt("  Diretorio base dos Contexts")
        .default("back/app/Contexts".to_string())
        .interact_text()?;

    let namespace_base: String = Input::with_theme(theme)
        .with_prompt("  Namespace base PHP")
        .default("App\\Contexts".to_string())
        .interact_text()?;

    let migration_path: String = Input::with_theme(theme)
        .with_prompt("  Diretorio das Migrations")
        .default("database/migrations".to_string())
        .interact_text()?;

    let erp_id: String = Input::with_theme(theme)
        .with_prompt("  ERP ID (tenant config)")
        .default("revlog-core-01".to_string())
        .interact_text()?;

    let company_name: String = Input::with_theme(theme)
        .with_prompt("  Nome da empresa")
        .default("Reversa Express Log".to_string())
        .interact_text()?;

    let warehouse_id: String = Input::with_theme(theme)
        .with_prompt("  ID do CD (warehouse)")
        .default("CD-SP-01".to_string())
        .interact_text()?;

    println!();
    println!("  {}", style("Gerando estrutura completa...").yellow().bold());
    println!();

    generators::logistica_reversa::generate(&generators::logistica_reversa::LogisticaReversaOptions {
        base_path,
        namespace_base,
        migration_path,
        erp_id,
        company_name,
        warehouse_id,
    })?;

    Ok(())
}

// ─── ERP Estoque Wizard ───────────────────────────────────────────────────────

fn wizard_estoque(theme: &ColorfulTheme) -> Result<()> {
    println!("{}", style("  [ Scaffold ERP Estoque DDD ]").yellow().bold());
    println!();
    println!("  {}", style("Gera 6 Contexts + 10 Migrations + 10 Models + UseCases Kardex + Manager JSON").dim());
    println!("  {}", style("Entidades: Armazem · Fornecedor · Produto · Lote").dim());
    println!("  {}", style("           PedidoCompra · MovimentacaoEstoque (Kardex) · Inventario").dim());
    println!();

    let base_path: String = Input::with_theme(theme)
        .with_prompt("  Diretorio base dos Contexts")
        .default("back/app/Contexts".to_string())
        .interact_text()?;

    let namespace_base: String = Input::with_theme(theme)
        .with_prompt("  Namespace base PHP")
        .default("App\\Contexts".to_string())
        .interact_text()?;

    let migration_path: String = Input::with_theme(theme)
        .with_prompt("  Diretorio das Migrations")
        .default("database/migrations".to_string())
        .interact_text()?;

    let tenant_id: String = Input::with_theme(theme)
        .with_prompt("  Tenant ID (config do manager)")
        .default("corp-estoque-001".to_string())
        .interact_text()?;

    let methods = vec![
        "CUSTO_MEDIO_PONDERADO  (Custo Medio — padrao)",
        "PEPS                   (Primeiro a Entrar, Primeiro a Sair)",
    ];
    let method_idx = Select::with_theme(theme)
        .with_prompt("  Metodo de custeio")
        .items(&methods)
        .default(0)
        .interact()?;

    let valuation_method = if method_idx == 0 { "CUSTO_MEDIO_PONDERADO" } else { "PEPS" };

    println!();
    println!("  {}", style("Gerando estrutura completa...").yellow().bold());
    println!();

    generators::estoque::generate(&generators::estoque::EstoqueOptions {
        base_path,
        namespace_base,
        migration_path,
        tenant_id,
        valuation_method: valuation_method.to_string(),
    })?;

    Ok(())
}

// ─── Docker Wizard ────────────────────────────────────────────────────────────

fn wizard_docker(theme: &ColorfulTheme) -> Result<()> {
    println!("{}", style("  [ Gerar Infra Docker ]").yellow().bold());
    println!();
    println!("  {}", style("Gera Dockerfile.dev + Dockerfile.prod + docker-compose (dev/prod)").dim());
    println!("  {}", style("+ Nginx + PHP-FPM + Xdebug + Supervisor + Makefile + .env.example").dim());
    println!();

    let app_name: String = Input::with_theme(theme)
        .with_prompt("  Nome da aplicacao")
        .default("MyApp".to_string())
        .interact_text()?;

    let output_path: String = Input::with_theme(theme)
        .with_prompt("  Diretorio de saida (raiz do projeto Laravel)")
        .default(".".to_string())
        .interact_text()?;

    let server_name: String = Input::with_theme(theme)
        .with_prompt("  server_name do Nginx (ex: localhost ou app.local)")
        .default("localhost".to_string())
        .interact_text()?;

    // ── PHP version ───────────────────────────────────────────────────────────
    let php_versions = vec!["8.3 (latest stable)", "8.2", "8.1"];
    let php_idx = Select::with_theme(theme)
        .with_prompt("  Versao do PHP")
        .items(&php_versions)
        .default(0)
        .interact()?;
    let php_version = match php_idx {
        0 => "8.3",
        1 => "8.2",
        _ => "8.1",
    };

    // ── Node version ──────────────────────────────────────────────────────────
    let node_versions = vec!["22 (LTS current)", "20 (LTS)"];
    let node_idx = Select::with_theme(theme)
        .with_prompt("  Versao do Node.js")
        .items(&node_versions)
        .default(0)
        .interact()?;
    let node_version: u8 = if node_idx == 0 { 22 } else { 20 };

    // ── Bancos de dados ───────────────────────────────────────────────────────
    let db_labels = &[
        "MySQL 8.4",
        "MariaDB 11.4",
        "PostgreSQL 17",
        "SQL Server 2022",
        "SQLite (sem container, apenas extensao)",
    ];
    let db_keys = &["mysql", "mariadb", "pgsql", "sqlserver", "sqlite"];
    let db_defaults = &[true, false, false, false, false];

    let db_selecionados = MultiSelect::with_theme(theme)
        .with_prompt("  Bancos de dados [Espaco=selecionar]")
        .items(db_labels)
        .defaults(db_defaults)
        .interact()?;

    let databases: Vec<String> = db_selecionados
        .iter()
        .map(|&i| db_keys[i].to_string())
        .collect();

    // ── Extras ────────────────────────────────────────────────────────────────
    let with_redis = Confirm::with_theme(theme)
        .with_prompt("  Incluir Redis (cache + queue + session)?")
        .default(true)
        .interact()?;

    let with_mailpit = Confirm::with_theme(theme)
        .with_prompt("  Incluir Mailpit (SMTP local para dev)?")
        .default(true)
        .interact()?;

    println!();
    println!("  {}", style("Gerando infra...").yellow().bold());
    println!();

    generators::docker::generate(&generators::docker::DockerOptions {
        app_name,
        php_version: php_version.to_string(),
        node_version,
        output_path,
        server_name,
        databases,
        with_redis,
        with_mailpit,
    })?;

    Ok(())
}


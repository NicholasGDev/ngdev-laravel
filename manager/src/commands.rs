//! Pontes entre o frontend DaisyUI e as camadas de aplicação do ngdev-laravel.
//! Cada função `#[tauri::command]` mapeia diretamente a um caso de uso do core.

use ngdev_laravel::{
    cli::{ControllerArgs, MigrationArgs, ModelArgs, PdvArgs},
    flows::{
        artesanal::{
            controller::generator::generate as gen_controller,
            migration::generator::generate as gen_migration,
            model::generator::generate as gen_model,
        },
        context::generator::{generate as gen_context, ContextOptions},
        docker::generator::{generate as gen_docker, DockerOptions},
        estoque::generator::{generate as gen_estoque, EstoqueOptions},
        landing_page::generator::{generate as gen_landing_page, LandingPageOptions},
        logistica_reversa::generator::{
            generate as gen_logistica_reversa, LogisticaReversaOptions,
        },
        pdv::generator::generate as gen_pdv,
    },
};
use serde::Deserialize;

// ── DTOs de entrada (deserializados do JSON do frontend) ─────────────────────

#[derive(Deserialize)]
pub struct ContextInput {
    pub nome: String,
    pub base_path: String,
    pub prefixo: String,
    pub namespace_base: String,
    pub com_entity: bool,
    pub com_autorizacoes: bool,
    pub operacoes: Vec<String>,
}

#[derive(Deserialize)]
pub struct DockerInput {
    pub app_name: String,
    pub php_version: String,
    pub node_version: u8,
    pub output_path: String,
    pub server_name: String,
    pub databases: Vec<String>,
    pub with_redis: bool,
    pub with_mailpit: bool,
}

#[derive(Deserialize)]
pub struct PdvInput {
    pub migrations_only: bool,
    pub models_only: bool,
    pub project_root: String,
}

#[derive(Deserialize)]
pub struct LandingPageInput {
    pub product_name: String,
    pub tagline: String,
    pub company_name: String,
    pub theme: String,
    pub layout: String,
    pub sections: Vec<String>,
    pub output_dir: String,
}

#[derive(Deserialize)]
pub struct EstoqueInput {
    pub base_path: String,
    pub namespace_base: String,
    pub migration_path: String,
    pub tenant_id: String,
    pub valuation_method: String,
}

#[derive(Deserialize)]
pub struct LogisticaReversaInput {
    pub base_path: String,
    pub namespace_base: String,
    pub migration_path: String,
    pub erp_id: String,
    pub company_name: String,
    pub warehouse_id: String,
}

#[derive(Deserialize)]
pub struct ControllerInput {
    pub name: String,
    pub resource: bool,
    pub model: Option<String>,
    pub project_root: String,
}

#[derive(Deserialize)]
pub struct ModelInput {
    pub name: String,
    pub migration: bool,
    pub controller: bool,
    pub project_root: String,
}

#[derive(Deserialize)]
pub struct MigrationInput {
    pub name: String,
    pub table: Option<String>,
    pub project_root: String,
}

// ── Comandos Tauri ────────────────────────────────────────────────────────────

/// Gera um DDD Context completo (Application / Domain / Infra).
#[tauri::command]
pub fn generate_context(input: ContextInput) -> Result<String, String> {
    let opts = ContextOptions {
        nome: input.nome.clone(),
        base_path: input.base_path,
        prefixo: input.prefixo,
        namespace_base: input.namespace_base,
        com_entity: input.com_entity,
        com_autorizacoes: input.com_autorizacoes,
        operacoes: input.operacoes,
    };
    gen_context(&opts)
        .map(|_| format!("Context '{}' gerado com sucesso.", input.nome))
        .map_err(|e| e.to_string())
}

/// Gera toda a infraestrutura Docker (Dockerfiles + docker-compose).
#[tauri::command]
pub fn generate_docker(input: DockerInput) -> Result<String, String> {
    let opts = DockerOptions {
        app_name: input.app_name.clone(),
        php_version: input.php_version,
        node_version: input.node_version,
        output_path: input.output_path,
        server_name: input.server_name,
        databases: input.databases,
        with_redis: input.with_redis,
        with_mailpit: input.with_mailpit,
    };
    gen_docker(&opts)
        .map(|_| format!("Docker para '{}' gerado com sucesso.", input.app_name))
        .map_err(|e| e.to_string())
}

/// Gera migrations e models do módulo PDV.
#[tauri::command]
pub fn generate_pdv(input: PdvInput) -> Result<String, String> {
    let args = PdvArgs {
        migrations_only: input.migrations_only,
        models_only: input.models_only,
        project_root: input.project_root,
    };
    gen_pdv(&args)
        .map(|_| "PDV gerado com sucesso.".to_string())
        .map_err(|e| e.to_string())
}

/// Gera a landing page em HTML com Tailwind + DaisyUI.
#[tauri::command]
pub fn generate_landing_page(input: LandingPageInput) -> Result<String, String> {
    let opts = LandingPageOptions {
        product_name: input.product_name.clone(),
        tagline: input.tagline,
        company_name: input.company_name,
        theme: input.theme,
        layout: input.layout,
        sections: input.sections,
        output_dir: input.output_dir.clone(),
    };
    gen_landing_page(&opts)
        .map(|_| format!("Landing page '{}' gerada em: {}", input.product_name, input.output_dir))
        .map_err(|e| e.to_string())
}

/// Gera o módulo completo de Estoque (6 Bounded Contexts + migrations + models).
#[tauri::command]
pub fn generate_estoque(input: EstoqueInput) -> Result<String, String> {
    let opts = EstoqueOptions {
        base_path: input.base_path,
        namespace_base: input.namespace_base,
        migration_path: input.migration_path,
        tenant_id: input.tenant_id,
        valuation_method: input.valuation_method,
    };
    gen_estoque(&opts)
        .map(|_| "Módulo de Estoque gerado com sucesso.".to_string())
        .map_err(|e| e.to_string())
}

/// Gera o módulo completo de Logística Reversa de Sinistros.
#[tauri::command]
pub fn generate_logistica_reversa(input: LogisticaReversaInput) -> Result<String, String> {
    let opts = LogisticaReversaOptions {
        base_path: input.base_path,
        namespace_base: input.namespace_base,
        migration_path: input.migration_path,
        erp_id: input.erp_id,
        company_name: input.company_name,
        warehouse_id: input.warehouse_id,
    };
    gen_logistica_reversa(&opts)
        .map(|_| "Módulo de Logística Reversa gerado com sucesso.".to_string())
        .map_err(|e| e.to_string())
}

/// Gera um Controller Laravel avulso.
#[tauri::command]
pub fn generate_controller(input: ControllerInput) -> Result<String, String> {
    let args = ControllerArgs {
        name: input.name.clone(),
        resource: input.resource,
        model: input.model,
        service: false,
        project_root: input.project_root,
    };
    gen_controller(&args)
        .map(|_| format!("Controller '{}' gerado com sucesso.", input.name))
        .map_err(|e| e.to_string())
}

/// Gera um Model Eloquent avulso (opcionalmente com migration e controller).
#[tauri::command]
pub fn generate_model(input: ModelInput) -> Result<String, String> {
    let args = ModelArgs {
        name: input.name.clone(),
        migration: input.migration,
        controller: input.controller,
        service: false,
        project_root: input.project_root,
    };
    gen_model(&args)
        .map(|_| format!("Model '{}' gerado com sucesso.", input.name))
        .map_err(|e| e.to_string())
}

/// Gera uma Migration Laravel avulsa.
#[tauri::command]
pub fn generate_migration(input: MigrationInput) -> Result<String, String> {
    let args = MigrationArgs {
        name: input.name.clone(),
        table: input.table,
        project_root: input.project_root,
    };
    gen_migration(&args)
        .map(|_| format!("Migration '{}' gerada com sucesso.", input.name))
        .map_err(|e| e.to_string())
}

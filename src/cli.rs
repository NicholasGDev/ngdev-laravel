use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ngdev")]
#[command(about = "Ng Development - Laravel PHP code generator")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate Laravel files (model, controller, migration)
    Make(MakeArgs),
}

#[derive(Args)]
pub struct MakeArgs {
    #[command(subcommand)]
    pub target: MakeTarget,
}

#[derive(Subcommand)]
pub enum MakeTarget {
    /// Generate an Eloquent model
    Model(ModelArgs),
    /// Generate a controller
    Controller(ControllerArgs),
    /// Generate a migration
    Migration(MigrationArgs),
    /// Scaffold all PDV (Ponto de Venda) migrations and models
    Pdv(PdvArgs),
}

#[derive(Args)]
pub struct PdvArgs {
    /// Generate only migrations (skip models)
    #[arg(long)]
    pub migrations_only: bool,
    /// Generate only models (skip migrations)
    #[arg(long)]
    pub models_only: bool,
}

#[derive(Args)]
pub struct ModelArgs {
    /// Model name (e.g. User, BlogPost)
    pub name: String,
    /// Also generate a migration
    #[arg(short = 'm', long)]
    pub migration: bool,
    /// Also generate a controller
    #[arg(short = 'c', long)]
    pub controller: bool,
}

#[derive(Args)]
pub struct ControllerArgs {
    /// Controller name (e.g. UserController)
    pub name: String,
    /// Generate a resource controller
    #[arg(short = 'r', long)]
    pub resource: bool,
    /// Bind to a model (e.g. User)
    #[arg(short = 'm', long)]
    pub model: Option<String>,
}

#[derive(Args)]
pub struct MigrationArgs {
    /// Migration name (e.g. create_users_table)
    pub name: String,
    /// Table to create
    #[arg(long)]
    pub table: Option<String>,
}

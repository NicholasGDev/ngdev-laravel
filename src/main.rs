mod cli;
mod generators;
mod templates;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Make(make) => match make.target {
            cli::MakeTarget::Model(args) => generators::model::generate(&args)?,
            cli::MakeTarget::Controller(args) => generators::controller::generate(&args)?,
            cli::MakeTarget::Migration(args) => generators::migration::generate(&args)?,
            cli::MakeTarget::Pdv(args) => generators::pdv::generate(&args)?,
        },
    }

    Ok(())
}

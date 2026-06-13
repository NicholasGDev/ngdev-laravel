pub mod generator;
pub mod templates;

use anyhow::Result;
use console::style;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};

pub fn run(theme: &ColorfulTheme) -> Result<()> {
    println!("{}", style("  [ Criar Controller ]").yellow().bold());
    println!();

    let project_root: String = Input::with_theme(theme)
        .with_prompt("  Caminho absoluto do projeto Laravel (ex: /home/user/meu-projeto)")
        .interact_text()?;

    let nome: String = Input::with_theme(theme)
        .with_prompt("  Nome do Controller (ex: UserController)")
        .interact_text()?;

    let com_service = Confirm::with_theme(theme)
        .with_prompt("  Gerar Service junto (MVCS)?")
        .default(true)
        .interact()?;

    let model_nome: String = Input::with_theme(theme)
        .with_prompt("  Vincular a um Model? (deixe em branco para pular)")
        .allow_empty(true)
        .interact_text()?;

    generator::generate(&crate::cli::ControllerArgs {
        name: nome,
        resource: true,
        model: if model_nome.trim().is_empty() { None } else { Some(model_nome) },
        service: com_service,
        project_root,
    })?;

    Ok(())
}

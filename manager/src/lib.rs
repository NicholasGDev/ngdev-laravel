mod commands;

/// Ponto de entrada da aplicação Tauri.
/// Registra todos os comandos e inicializa a janela.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::generate_context,
            commands::generate_docker,
            commands::generate_pdv,
            commands::generate_landing_page,
            commands::generate_estoque,
            commands::generate_logistica_reversa,
            commands::generate_controller,
            commands::generate_model,
            commands::generate_migration,
        ])
        .run(tauri::generate_context!())
        .expect("Erro ao inicializar o caronte Manager");
}

// Ponto de entrada — mantém windows sem console em builds release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    caronte_manager_lib::run()
}

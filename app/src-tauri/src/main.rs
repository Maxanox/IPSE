// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

mod core;
use core::app_system::simulation::manager::{SimulationManager, select_simulation_template, initialize_simulation, run_simulation, stop_simulation, quit_simulation};

fn main() -> Result<(), tauri::Error> {
  let simulation_manager = Arc::new(Mutex::new(SimulationManager::new()));
  tauri::Builder::default()
    .manage(simulation_manager)
    .invoke_handler(tauri::generate_handler![select_simulation_template, initialize_simulation, run_simulation, stop_simulation, quit_simulation])
    .run(tauri::generate_context!())
}

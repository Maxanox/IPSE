// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

mod simulation;
use simulation::simulation_manager::{SimulationManager, run_simulation, stop_simulation, select_simulation_template};

fn main() -> Result<(), tauri::Error> {
  let simulation_manager = Arc::new(Mutex::new(SimulationManager::new()));
  tauri::Builder::default()
    .manage(simulation_manager)
    .invoke_handler(tauri::generate_handler![run_simulation, stop_simulation, select_simulation_template])
    .run(tauri::generate_context!())
}

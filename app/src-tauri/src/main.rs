// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::time::{Instant, Duration};
use std::sync::{Arc, Mutex};
use app::physics::fluid;
use serde::Serialize;

mod custom_maths;

use custom_maths::vector2::Vector2;
use custom_maths::utils::sign_f32;

mod utils;

use tauri::Manager;
use utils::payload_models::RenderPayload;

mod physics;

use physics::fluid::FluidParticle;

mod simulations;

use simulations::fluid_simulation::FluidSimulation;
use simulations::renderer::Renderer;

#[tauri::command]
async fn add_particles(fluid_simulation_mutex: tauri::State<'_, Arc<Mutex<FluidSimulation>>>, particles: Vec<Vector2>) -> Result<(), ()> {
  println!("Adding {} particles", particles.len());

  let mut fluid_simulation = fluid_simulation_mutex.lock().unwrap();

  for particle in particles {
    fluid_simulation.fluid.push_particle(FluidParticle::new(particle.x, particle.y));
  }
   
  Ok(())
}

#[tauri::command]
async fn start_simulation(app_handle: tauri::AppHandle, window: tauri::Window, width: f32, height: f32) -> Result<(), ()> {
  println!("Starting simulation");

  let fluid_simulation = Arc::new(Mutex::new(FluidSimulation::new(1.0, 128.0*0.3/2.0, width, height)));

  app_handle.manage(fluid_simulation.clone());
  
  std::thread::spawn(move || -> Result<(), ()> {
    loop {
        // When simulation paused
        while !fluid_simulation.lock().unwrap().running {
          std::thread::sleep(std::time::Duration::from_millis(100));
        }

        fluid_simulation.lock().unwrap().next_step();
        window.emit("next-step", ()).unwrap();

        println!("Simulation running with {} particles", fluid_simulation.lock().unwrap().fluid.particles.len());
        std::thread::sleep(std::time::Duration::from_millis(1000/120));
    }
  });

  Ok(())
}

#[tauri::command]
async fn pause_simulation(fluid_simulation_mutex: tauri::State<'_, Arc<Mutex<FluidSimulation>>>) -> Result<(), ()> {
  let mut fluid_simulation = fluid_simulation_mutex.lock().unwrap();

  fluid_simulation.running = !fluid_simulation.running;

  if fluid_simulation.running {
    println!("Resuming simulation");
  } else {
    println!("Pausing simulation");
  }

  Ok(())
}

fn main() -> Result<(), tauri::Error> {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![add_particles, start_simulation, pause_simulation])
    .run(tauri::generate_context!())
}

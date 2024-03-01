// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::{Instant, Duration};
use std::sync::Mutex;
use serde::Serialize;

mod custom_maths;

use custom_maths::vector2::Vector2;
use custom_maths::utils::sign_f32;

mod utils;

use utils::payload_models::RenderPayload;

#[derive(Clone, Serialize)]
struct Simulation {
  positions: Vec<Vector2>,
  velocities: Vec<Vector2>,
  #[serde(skip_serializing)]
  last_update: Instant,
  dt: Duration,
  particle_size: f32
}

impl Simulation {
  fn new() -> Simulation {
    println!("Simulation created");
    Simulation {
      positions: Vec::new(),
      velocities: Vec::new(),
      last_update: Instant::now(),
      dt: Duration::from_secs(0),
      particle_size: 64.0 * 0.1,
    }
  }

  fn run(&mut self, window: &tauri::Window, width: f32, height: f32) -> () {
    self.last_update = Instant::now();
    loop {
      //let target_fps = 60.0;
      //let frame_time = Duration::from_secs_f32(1.0 / target_fps);
      //let sleep_time = frame_time.checked_sub(self.dt).unwrap_or_default();
      //std::thread::sleep(sleep_time);

      std::thread::sleep(Duration::from_millis(1000 / 60));

      let now = Instant::now();
      self.dt = now.duration_since(self.last_update);
      self.last_update = now;
      
      match window.emit("next-step", ())
      {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e),
      }

      self.update();
      self.resolve_collision(width, height);
      self.draw(window);
    }
  }

  fn resolve_collision(&mut self, width: f32, height: f32) -> () {
    let n: usize = self.positions.len();

    for i in 0..n {
      if self.positions[i].x.abs()  + self.particle_size > width {
        self.positions[i].x = width - self.particle_size * sign_f32(self.positions[i].x);
        self.velocities[i].x *= -1.0;
      }

      if self.positions[i].y.abs() + self.particle_size > height {
        self.positions[i].y = height - self.particle_size * sign_f32(self.positions[i].y);
        self.velocities[i].y *= -1.0 * 0.8;
      }
    }
  }

  fn update(&mut self) -> () {
    let n: usize = self.positions.len();
    for i in 0..n {
      self.velocities[i] += Vector2::down() * 9.81 * self.dt.as_secs_f32() * 30.0;
      self.positions[i] += self.velocities[i] * self.dt.as_secs_f32();
    }
  }

  fn draw(&mut self, window: &tauri::Window) -> () {
    let payload = RenderPayload {
      positions: self.positions.clone(),
    };

    window.emit("draw-particles", payload).unwrap();
  }

  fn add_particle(&mut self, x: f32, y: f32) -> () {
    self.positions.push(Vector2::new(x, y));
    self.velocities.push(Vector2::zero());
  }
}

#[tauri::command]
async fn start_simulation(window: tauri::Window, simulation: tauri::State<'_, Mutex<Simulation>>, width: f32, height: f32) -> Result<(), ()> {
  println!("Starting simulation");
  let mut simulation = simulation.lock().unwrap();
  simulation.run(&window, width, height);

  Ok(())
}

#[tauri::command]
async fn add_particles(simulation: tauri::State<'_, Mutex<Simulation>>, particles: Vec<Vector2>) -> Result<(), ()> {
  println!("Adding particles");
  let mut simulation = simulation.lock().unwrap();
  for particle in particles {
    simulation.add_particle(particle.x, particle.y);
  }
   
  Ok(())
}

fn main() -> Result<(), tauri::Error> {
  tauri::Builder::default()
    .manage(Mutex::new(Simulation::new()))
    .invoke_handler(tauri::generate_handler![start_simulation, add_particles])
    .run(tauri::generate_context!())
}

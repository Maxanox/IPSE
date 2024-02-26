// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::{Instant, Duration};
use std::sync::Mutex;
use std::ops::{Add, Mul, Sub, AddAssign, SubAssign, MulAssign};
use serde::{Serialize, Deserialize};

// Retourne le signe d'un nombre
fn sign(x: f32) -> f32 {
  match x {
      x if x > 0.0 => 1.0,
      x if x < 0.0 => -1.0,
      _ => 0.0,
  }
}

#[derive(Copy, Serialize, Deserialize, Clone)]
struct Vector2 {
  x: f32,
  y: f32,
}

#[allow(dead_code)]
impl Vector2 {
  // Constructeur de base
  fn new(x: f32, y: f32) -> Vector2 {
    Vector2 { x, y }
  }

  // Constructeur de vecteur nul
  fn zero() -> Vector2 {
    Vector2 { x: 0.0, y: 0.0 }
  }

  // Constructeur du vecteur droite
  fn right() -> Vector2 {
    Vector2 { x: 1.0, y: 0.0 }
  }

  // Constructeur du vecteur gauche
  fn left() -> Vector2 {
    Vector2 { x: -1.0, y: 0.0 }
  }

  // vecteur haut et bas, sont inversés pour correspondre
  // à la convention de coordonnées du canvas utilisé par PIXI.js

  // Constructeur du vecteur haut
  fn up() -> Vector2 {
    Vector2 { x: 0.0, y: -1.0 }
  }

  // Constructeur du vecteur bas
  fn down() -> Vector2 {
    Vector2 { x: 0.0, y: 1.0 }
  }

  // Retourne le vecteur opposé
  fn opposite(&self) -> Vector2 {
    Vector2 {
      x: -self.x,
      y: -self.y,
    }
  }

  // Retourne la norme du vecteur
  fn magnitude(&self) -> f32 {
    (self.x * self.x + self.y * self.y).sqrt()
  }

  // Retourne le vecteur normalisé
  fn normalize(&self) -> Vector2 {
    let magnitude = self.magnitude();
    Vector2 {
      x: self.x / magnitude,
      y: self.y / magnitude,
    }
  }
}

// Implementation de l'addition
impl Add<Vector2> for Vector2 {
  type Output = Vector2;

  fn add(self, rhs: Vector2) -> Self::Output {
    Vector2 {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    }
  }
}

// Implementation de la soustraction
impl Sub<Vector2> for Vector2 {
  type Output = Vector2;

  fn sub(self, rhs: Vector2) -> Self::Output {
    Vector2 {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
    }
  }
}

// Implementation de l'addition avec assignation
impl AddAssign<Vector2> for Vector2 {
  fn add_assign(&mut self, rhs: Vector2) {
    self.x += rhs.x;
    self.y += rhs.y;
  }
}

// Implementation de la soustraction avec assignation
impl SubAssign<Vector2> for Vector2 {
  fn sub_assign(&mut self, rhs: Vector2) {
    self.x -= rhs.x;
    self.y -= rhs.y;
  }
}

// Implementation du produit scalaire
impl Mul<Vector2> for Vector2 {
  type Output = f32;

  fn mul(self, rhs: Vector2) -> Self::Output {
    self.x * rhs.x + self.y * rhs.y
  }
}

// Implementation de la multiplication par un coefficient
impl Mul<f32> for Vector2 {
  type Output = Vector2;

  fn mul(self, rhs: f32) -> Self::Output {
    Vector2 {
      x: self.x * rhs,
      y: self.y * rhs,
    }
  }
}

// Implementation de la multiplication par un coefficient avec assignation
impl MulAssign<f32> for Vector2 {
  fn mul_assign(&mut self, rhs: f32) -> () {
    self.x *= rhs;
    self.y *= rhs;
  }
}

#[derive(Clone, Serialize)]
struct RenderPayload {
  positions: Vec<Vector2>,
}

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

      std::thread::sleep(Duration::from_millis(1000 / 80));

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
        self.positions[i].x = width - self.particle_size * sign(self.positions[i].x);
        self.velocities[i].x *= -1.0;
      }

      if self.positions[i].y.abs() + self.particle_size > height {
        self.positions[i].y = height - self.particle_size * sign(self.positions[i].y);
        self.velocities[i].y *= -1.0 * 1.0;
      }
    }
  }

  fn update(&mut self) -> () {
    let n: usize = self.positions.len();

    for i in 0..n {
      self.velocities[i] += Vector2::down() * 9.81 * self.dt.as_secs_f32() * 15.0;
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
async fn start_simulation(window: tauri::Window, simulation: tauri::State<'_, Mutex<Simulation>>, width: f32, height: f32, particles: Vec<Vector2>) -> Result<(), ()> {
  println!("Adding particles");
  dbg!(particles.len());
  let mut simulation = simulation.lock().unwrap();
  for particle in particles {
    simulation.add_particle(particle.x, particle.y);
  }
  println!("Starting simulation");
  simulation.run(&window, width, height);

  Ok(())
}

fn main() -> Result<(), tauri::Error> {
  tauri::Builder::default()
    .manage(Mutex::new(Simulation::new()))
    .invoke_handler(tauri::generate_handler![start_simulation])
    .run(tauri::generate_context!())
}

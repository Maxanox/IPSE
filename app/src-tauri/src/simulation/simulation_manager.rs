use std::time::Instant;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};

use crate::simulation::renderer::Renderer;
use crate::simulation::simulation_template::SimulationTemplate;
use crate::simulation::frame_history::FrameHistory;
use crate::simulation::templates::bouncing_balls::BouncingBallSimulation;

use super::custom_maths::vector2::Vector2;

/// The `Manager` struct represents a simulation manager.
/// It is responsible for managing the simulation, updating the delta time,
/// and controlling the simulation's state.
pub struct SimulationManager {
    renderer: Option<Renderer>,
    simulation: Option<Box<dyn SimulationTemplate>>,
    delta_time: f32,
    last_delta_time_update: Instant,
    is_running: bool,
    frame_history: FrameHistory<f32>
}

impl SimulationManager {
    /// Creates a new `Manager` instance.
    pub fn new() -> Self {
        SimulationManager {
            renderer: None,
            simulation: None,
            delta_time: 0.0,
            last_delta_time_update: Instant::now(),
            is_running: false,
            frame_history: FrameHistory::new(10.0, 4.0)
        }
    }

    /// Updates the delta time based on the current time.
    fn update_delta_time(&mut self) {
        let now = Instant::now();
        self.delta_time = now.duration_since(self.last_delta_time_update).as_secs_f32();
        self.last_delta_time_update = now;
    }

    /// Resets the manager by clearing the simulation and resetting the delta time.
    fn reset(&mut self) {
        self.simulation = None;
        self.delta_time = 0.0;
        self.last_delta_time_update = Instant::now();
    }

    /// Sets the simulation template for the manager.
    ///
    /// This method sets the simulation template for the manager and resets the manager's state.
    ///
    /// # Arguments
    ///
    /// * `simulation` - A boxed trait object representing the simulation template.
    pub fn set_simulation_template(&mut self, simulation: Box<dyn SimulationTemplate>) {
        self.reset();
        self.simulation = Some(simulation);
    }

    /// Sets the running state of the manager.
    ///
    /// If the simulation is resumed after a pause, the last delta time is updated
    /// to exclude the time spent in pause.
    /// 
    /// # Arguments
    /// * `v` - A boolean value indicating whether the simulation is running.
    pub fn set_running(&mut self, v: bool) {
        if v {
            self.last_delta_time_update = Instant::now();
        };

        self.is_running = v;
    }

    /// Gets the running state of the manager.
    pub fn get_running(&self) -> bool {
        self.is_running
    }

    /// Performs the next step of the simulation.
    ///
    /// # Errors
    ///
    /// Returns an error if no simulation template is set.
    pub fn performs(&mut self) -> Result<(), String> {
        self.update_delta_time();
        match self.simulation.as_mut() {
            Some(simulation) => simulation.next_step(self.delta_time),
            None => Err("No simulation template set".to_string()),
        }
    }

    /// Moves the simulation forward by the specified number of steps.
    ///
    /// # Panics
    ///
    /// This method is not implemented and will panic if called.
    pub fn forward(&mut self, steps: Option<u32>) {
        unimplemented!()
    }

    /// Moves the simulation backward by the specified number of steps.
    ///
    /// # Panics
    ///
    /// This method is not implemented and will panic if called.
    pub fn backward(&mut self, steps: Option<u32>) {
        unimplemented!()
    }

    /// Renders the simulation.
    ///
    /// This method renders the simulation using the current renderer,
    /// by retrieving the data associated with the simulation template to the front-end.
    ///
    /// # Errors
    ///
    /// Returns an error if no simulation template is set or if no renderer is set.
    pub fn render(&self) -> Result<(), String> {
        match self.simulation.as_ref() {
            Some(simulation) => {

                let data = simulation.get_renderer_data().unwrap();

                match self.renderer.as_ref() {
                    Some(renderer) => renderer.render(data),
                    None => Err("No renderer set".to_string())
                }
            },
            None => Err("No simulation template set".to_string())
        }
    }
}

#[tauri::command]
pub async fn run_simulation(simulation_manager: tauri::State<'_, Arc<Mutex<SimulationManager>>>) -> Result<(), String> {
    println!("Running simulation...");
    match simulation_manager.lock() {
        Ok(mut simulation_manager) => simulation_manager.set_running(true),
        Err(e) => return Err(e.to_string())
    };

    let simulation_manager = Arc::clone(&simulation_manager);

    let join_handler = std::thread::spawn(move || -> Result<(), String> {
        while match simulation_manager.lock() {Ok(simulation_manager) => simulation_manager.get_running(), Err(e) => return Err(e.to_string())} 
        {
            match simulation_manager.lock() {
                Ok(mut simulation_manager) => {
                    simulation_manager.performs()?;
                    simulation_manager.render()?;
                },
                Err(e) => return Err(e.to_string())
            };

            std::thread::sleep(std::time::Duration::from_millis(1000/60));
        }

        Ok(())
    });

    match join_handler.join() {
        Ok(result) => result,
        Err(_) => Err("Error joining thread".to_string())
    }
}

#[tauri::command]
pub async fn stop_simulation(simulation_manager: tauri::State<'_, Arc<Mutex<SimulationManager>>>) -> Result<(), String> {
    match simulation_manager.lock() {
        Ok(mut simulation_manager) => simulation_manager.set_running(false),
        Err(e) => return Err(e.to_string())
    };

    Ok(())
}

#[derive(Serialize, Deserialize)]
pub enum SimulationTemplateEnum {
    BouncingBalls
}

#[tauri::command]
pub async fn select_simulation_template(window: tauri::Window, simulation_manager: tauri::State<'_, Arc<Mutex<SimulationManager>>>, width: f32, height: f32) -> Result<(), String> {
    println!("Simulation template selecting...");

    let renderer = Renderer::new(Vector2::new(width, height), window);
    let gradient = match colorgrad::CustomGradient::new().html_colors(&["#0077ff", "#24ff6f", "ffff20", "ff3131"]).domain(&[0.0, 0.5, 0.7, 1.0]).build() {
        Ok(gradient) => gradient,
        Err(e) => return Err(e.to_string())
    };

    let mut selected_template = BouncingBallSimulation::new(renderer.size, gradient, None, None, None, None);

    selected_template.add_ball();

    match simulation_manager.lock() {
        Ok(mut simulation_manager) => {
            simulation_manager.set_simulation_template(Box::new(selected_template));
            simulation_manager.renderer = Some(renderer);
        },
        Err(e) => return Err(e.to_string())
    };

    println!("Simulation template selected");
    Ok(())
}
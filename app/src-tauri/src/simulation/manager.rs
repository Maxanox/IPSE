use std::time::Instant;
use std::sync::{Arc, Mutex};

use crate::simulation::template::Template;
use crate::simulation::frame_history::FrameHistory;

/// The `Manager` struct represents a simulation manager.
/// It is responsible for managing the simulation, updating the delta time,
/// and controlling the simulation's state.
pub struct Manager {
    simulation: Option<Box<dyn Template>>,
    delta_time: f32,
    last_delta_time_update: Instant,
    is_running: bool,
    frame_history: FrameHistory<f32>,
}

impl Manager {
    /// Creates a new `Manager` instance.
    pub fn new() -> Self {
        Manager {
            simulation: None,
            delta_time: 0.0,
            last_delta_time_update: Instant::now(),
            is_running: false,
            frame_history: FrameHistory::new(10.0, 4.0),
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
    pub fn set_simulation_template(&mut self, simulation: Box<dyn Template>) {
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
}

#[tauri::command]
async fn run(window: tauri::Window, simulation_manager: tauri::State<'_, Arc<Mutex<Manager>>>) -> Result<(), String> {
    
    match simulation_manager.lock() {
        Ok(mut simulation_manager) => simulation_manager.set_running(true),
        Err(e) => return Err(e.to_string())
    };

    let simulation_manager = Arc::clone(&simulation_manager);
    
    std::thread::spawn(move || -> Result<(), String> {
        while simulation_manager.lock().unwrap().is_running {

            match simulation_manager.lock() {
                Ok(mut simulation_manager) => simulation_manager.performs()?,
                Err(e) => return Err(e.to_string())
            };

            
            match window.emit("render", "payload") {
                Ok(_) => {},
                Err(e) => return Err(e.to_string())
            };

            std::thread::sleep(std::time::Duration::from_millis(1000/60));
        }

        Ok(())
    });

    Ok(())
}

#[tauri::command]
async fn stop(simulation_manager: tauri::State<'_, Arc<Mutex<Manager>>>) -> Result<(), String> {
    match simulation_manager.lock() {
        Ok(mut simulation_manager) => simulation_manager.set_running(false),
        Err(e) => return Err(e.to_string())
    };

    Ok(())
}

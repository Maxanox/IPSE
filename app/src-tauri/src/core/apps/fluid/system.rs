use crate::core::app_system::simulation::template::SimulationTemplate;
use crate::core::app_system::simulation::renderer::DataToSend;
use crate::core::sciences::maths::vector2::Vector2;

use super::main::Fluid;
use super::data::*;

/// Obligatory implementation of the `SimulationTemplate` trait for the `BouncingBallSimulation` struct.
impl SimulationTemplate for Fluid {
    fn initialize(&mut self, renderer_size: Vector2, serialized_data: Option<String>) -> Result<(), String> {
        self.box_bound_x = renderer_size.x;
        self.box_bound_y = renderer_size.y;

        let starter_data: StarterData = match serialized_data {
            Some(data) => match serde_json::from_str(&data) {
                Ok(deserialized_data) => deserialized_data,
                Err(e) => return Err(e.to_string())
            },
            None => return Ok(())
        };

        for position in starter_data.positions {
            self.particles.push(position);
        }

        Ok(())
    }

    fn next_step(&mut self, dt: f32) -> Result<(), String> {
        self.update(dt);

        Ok(())
    }

    fn get_data_to_render(&self) -> Result<Box<dyn DataToSend>, String> {
        let data_to_render = RendererData {
            fluid_particles: self.particles.clone() // Use the clone method is not the most efficient way to do this, but it is the simplest way to implement it for now.
        };
        
        Ok(Box::new(data_to_render))
    }

    fn event_handler(&mut self, event: String, data: Option<String>) -> Result<(), String> {
        match event.as_str() {
            "set_settings" => {
                match data {
                    Some(data) => {
                        let settings: EventSettings = match serde_json::from_str(&data) {
                            Ok(deserialized_data) => deserialized_data,
                            Err(e) => return Err(e.to_string())
                        };

                        self.collision_restitution = settings.collision_restitution;
                        self.gravity = settings.gravity;
                        self.particles.target_density = settings.target_density;
                        self.particles.pressure_multiplier = settings.pressure_stiffness;
                        self.visual_filter = settings.visual_filter;
                        self.particles.smoothing_radius = settings.smoothing_radius;
                        self.viscosity_strength = settings.viscosity_strength;
                    },
                    None => return Err("No data provided for the 'set_settings' event".to_string())
                }

                if self.visual_filter == 0 {
                    for color in self.particles.colors.iter_mut() {
                        *color = "#FFFFFFFF".to_string();
                    }
                }
            },
            "interractive_force_toggle" => {
                match data {
                    Some(data) => {
                        let interactive_force_toggle: bool = match serde_json::from_str(&data) {
                            Ok(deserialized_data) => deserialized_data,
                            Err(e) => return Err(e.to_string())
                        };

                        self.interactive_force = interactive_force_toggle;
                    },
                    None => return Err("No data provided for the 'interractive_force_toggle' event".to_string())
                }
            }
            "interactive_force_position" => {
                match data {
                    Some(data) => {
                        let interactive_force_position: Vector2 = match serde_json::from_str(&data) {
                            Ok(deserialized_data) => deserialized_data,
                            Err(e) => return Err(e.to_string())
                        };

                        self.interactive_force = true;
                        self.interactive_force_position = interactive_force_position;
                    },
                    None => return Err("No data provided for the 'interactive_force' event".to_string())
                }
            },
            _ => return Err("Unknown event".to_string())
        }

        Ok(())
    }
}
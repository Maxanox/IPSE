use crate::core::app_system::simulation::template::SimulationTemplate;
use crate::core::app_system::simulation::renderer::DataToSend;
use crate::core::sciences::maths::vector2::Vector2;

use super::main::BouncingBallSimulation;
use super::data::*;

/// Obligatory implementation of the `SimulationTemplate` trait for the `BouncingBallSimulation` struct.
impl SimulationTemplate for BouncingBallSimulation {
    fn initialize(&mut self, renderer_size: Vector2, serialized_data: Option<String>) -> Result<(), String> {
        self.renderer_size = renderer_size;

        let starter_data: StarterData = match serialized_data {
            Some(data) => match serde_json::from_str(&data) {
                Ok(deserialized_data) => deserialized_data,
                Err(e) => return Err(e.to_string())
            },
            None => return Ok(())
        };

        for position in starter_data.positions {
            let ball = Ball::new(
                position, self.default_velocity, self.default_radius, self.default_mass, self.default_color.clone()
            );
            self.push_ball(ball);
        }

        Ok(())
    }

    fn next_step(&mut self, dt: f32) -> Result<(), String> {
        for ball in &mut self.balls {
            // Apply gravity
            ball.velocity += Vector2::down() * 9.81 * ball.mass * dt;
            // Update position
            ball.position += ball.velocity * dt;
            // Check for collision with the renderer bounds
            if ball.position.x - ball.radius < 0.0 {
                ball.position.x = ball.radius;
                ball.velocity.x *= -0.8;
            }
            if ball.position.x + ball.radius > self.renderer_size.x {
                ball.position.x = self.renderer_size.x - ball.radius;
                ball.velocity.x *= -0.8;
            }
            if ball.position.y - ball.radius < 0.0 {
                ball.position.y = ball.radius;
                ball.velocity.y *= -0.8;
            }
            if ball.position.y + ball.radius > self.renderer_size.y {
                ball.position.y = self.renderer_size.y - ball.radius;
                ball.velocity.y *= -0.8;
            }
            // Update color in function of the velocity
            let normalized_velocity = ball.velocity.magnitude() / 1000.0;
            ball.color = self.velocity_gradient.at(normalized_velocity as f64).to_hex_string();
        }

        Ok(())
    }

    fn get_data_to_render(&self) -> Result<Box<dyn DataToSend>, String> {
        let data_to_render = RendererData {
            balls: self.balls.clone()
        };
        
        Ok(Box::new(data_to_render))
    }
}
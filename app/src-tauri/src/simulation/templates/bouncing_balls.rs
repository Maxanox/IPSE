use serde::{Deserialize, Serialize};

use colorgrad::{Color, Gradient};

use crate::simulation::simulation_template::SimulationTemplate;
use crate::simulation::custom_maths::vector2::Vector2;
use crate::simulation::renderer::RendererData;

#[derive(Serialize, Clone)]
pub struct Ball {
    position: Vector2,
    velocity: Vector2,
    radius: f32,
    mass: f32,
    color: String, // store in hex format and not in colorgrad::Color to allow serialization
}

impl Ball {
    pub fn new(position: Vector2, velocity: Vector2, radius: f32, mass: f32, color: Color) -> Self {
        Ball {
            position,
            velocity,
            radius,
            mass,
            color: color.to_hex_string()
        }
    }
}

impl RendererData for Vec<Ball> {}

#[derive(Deserialize, Clone)]
struct BouncingBallStarterData {
    positions: Vec<Vector2>
}

/// Represents a bouncing ball simulation.
/// 
/// This struct holds information about the bouncing balls in the simulation.
/// It provides methods to create a new simulation, push a ball into the simulation,
/// and append multiple balls to the simulation.
/// 
/// In this simulation, radius and mass are equivalent, but we are using two fields for the code/logic comprehension.
pub struct BouncingBallSimulation {
    renderer_size: Vector2,
    balls: Vec<Ball>,
    velocity_gradient: Gradient,
    default_position: Vector2,
    default_velocity: Vector2,
    default_radius: f32,
    default_mass: f32,
    default_color: Color
}

/// Obligatory implementation of the `SimulationTemplate` trait for the `BouncingBallSimulation` struct.
impl SimulationTemplate for BouncingBallSimulation {
    fn initialize(&mut self, renderer_size: Vector2, serialized_data: Option<String>) -> Result<(), String> {
        self.renderer_size = renderer_size;

        let starter_data: BouncingBallStarterData = match serialized_data {
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

    fn get_data_to_render(&self) -> Result<Box<dyn RendererData>, String> {
        Ok(Box::new(self.balls.clone()))
    }
}

/// Custom implementation of the `BouncingBallSimulation` struct.
impl BouncingBallSimulation {

    /// Creates a new `BouncingBallSimulation` instance with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `renderer` - The renderer used to render the simulation.
    /// * `velocity_gradient` - The gradient used to determine the velocity of the balls.
    /// * `default_position` - The default position of the balls. If `None`, the default position is `Vector2::zero()`.
    /// * `default_velocity` - The default velocity of the balls. If `None`, the default velocity is `Vector2::zero()`.
    /// * `default_radius` - The default radius of the balls. If `None`, the default radius is `1.0`.
    /// * `default_color` - The default color of the balls. If `None`, the default color is determined by the velocity gradient at 0.0.
    ///
    /// # Returns
    ///
    /// A new `BouncingBallSimulation` instance.
    pub fn new(renderer_size: Vector2, velocity_gradient: Gradient, default_position: Option<Vector2>, default_velocity: Option<Vector2>, 
        default_radius: Option<f32>, default_color: Option<Color>) -> Self {
        let radius = if let Some(radius) = default_radius {radius} else {15.0};
        BouncingBallSimulation {
            renderer_size,
            balls: Vec::new(),
            default_position: if let Some(position) = default_position {position} else {Vector2::new(renderer_size.x / 2.0, renderer_size.y / 2.0)},
            default_velocity: if let Some(velocity) = default_velocity {velocity} else {Vector2::zero()},
            default_radius: radius,
            default_mass: radius*5.0,
            default_color: if let Some(color) = default_color {color} else {velocity_gradient.at(0.0)},
            velocity_gradient,
        }
    }

    /// Creates a new ball with the default position, velocity, radius, mass, and color, and adds it to the simulation.
    pub fn add_ball(&mut self) {
        let ball = Ball::new(
            self.default_position, self.default_velocity, self.default_radius, self.default_mass, self.default_color.clone()
        );

        self.push_ball(ball);
    }

    /// Pushes a ball into the simulation.
    ///
    /// # Arguments
    ///
    /// * `ball` - The ball to be pushed into the simulation.
    pub fn push_ball(&mut self, ball: Ball) {
        self.balls.push(ball);
    }

    /// Appends multiple balls to the simulation.
    ///
    /// # Arguments
    ///
    /// * `balls` - A mutable reference to a vector of balls to be appended to the simulation.
    pub fn append_balls(&mut self, balls: &mut Vec<Ball>) {
        self.balls.append(balls);
    }
}
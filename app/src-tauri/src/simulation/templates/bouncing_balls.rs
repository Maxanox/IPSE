use colorgrad::{Color, Gradient};

use crate::simulation::template::Template;
use crate::custom_maths::vector2::Vector2;
use crate::simulation::renderer::RendererData;

pub struct Ball {
    position: Vector2,
    velocity: Vector2,
    radius: f32,
    mass: f32,
    color: Color,
}

impl Ball {
    pub fn new(position: Vector2, velocity: Vector2, radius: f32, mass: f32, color: Color) -> Self {
        Ball {
            position,
            velocity,
            radius,
            mass,
            color
        }
    }
}

/// Represents a bouncing ball simulation.
/// 
/// This struct holds information about the bouncing balls in the simulation.
/// It provides methods to create a new simulation, push a ball into the simulation,
/// and append multiple balls to the simulation.
/// 
/// In this simulation, radius and mass are equivalent, but we are using two fields for the code/logic comprehension.
pub struct BouncingBallSimulation {
    balls: Vec<Ball>,
    velocity_gradient: Gradient,
    default_position: Vector2,
    default_velocity: Vector2,
    default_radius: f32,
    default_mass: f32,
    default_color: Color
}

impl BouncingBallSimulation {

    /// Creates a new `BouncingBallSimulation` instance with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `velocity_gradient` - The gradient used to determine the velocity of the balls.
    /// * `default_position` - The default position of the balls. If `None`, the default position is `Vector2::zero()`.
    /// * `default_velocity` - The default velocity of the balls. If `None`, the default velocity is `Vector2::zero()`.
    /// * `default_radius` - The default radius of the balls. If `None`, the default radius is `1.0`.
    /// * `default_color` - The default color of the balls. If `None`, the default color is determined by the velocity gradient at 0.0.
    ///
    /// # Returns
    ///
    /// A new `BouncingBallSimulation` instance.
    pub fn new(velocity_gradient: Gradient, default_position: Option<Vector2>, default_velocity: Option<Vector2>, 
        default_radius: Option<f32>, default_color: Option<Color>) -> Self {
        let radius = if let Some(radius) = default_radius {radius} else {1.0};
        BouncingBallSimulation {
            balls: Vec::new(),
            default_position: if let Some(position) = default_position {position} else {Vector2::zero()},
            default_velocity: if let Some(velocity) = default_velocity {velocity} else {Vector2::zero()},
            default_radius: radius,
            default_mass: radius,
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

impl Template for BouncingBallSimulation {
    fn next_step(&mut self, dt: f32) -> Result<(), String> {
        for ball in &mut self.balls {
            // Apply gravity
            ball.velocity += Vector2::down() * 9.81 * ball.mass * dt;
            // Update position
            ball.position += ball.velocity * dt;
            // Check for collision with the renderer bounds
            if ball.position.x - ball.radius < 0.0 {
                ball.position.x = ball.radius;
                ball.velocity.x *= -1.0;
            } else if ball.position.x + ball.radius > self.renderer.width {
                ball.position.x = self.renderer.width - ball.radius;
                ball.velocity.x *= -1.0;

            }
            if ball.position.y - ball.radius < 0.0 {
                ball.position.y = ball.radius;
                ball.velocity.y *= -1.0;
            } else if ball.position.y + ball.radius > self.renderer.height {
                ball.position.y = self.renderer.height - ball.radius;
                ball.velocity.y *= -1.0;
            }
            // Update color in function of the velocity
            let normalized_velocity = ball.velocity.magnitude() / 100.0;
            ball.color = self.velocity_gradient.at(normalized_velocity as f64);
        }

        Ok(())
    }

    fn get_renderer_data(&self) -> Box<dyn RendererData> {
        unimplemented!()
    }
}
use colorgrad::{Color, Gradient};
use super::data::*;
use crate::core::sciences::maths::vector2::Vector2;

/// Represents a bouncing ball simulation.
/// 
/// This struct holds information about the bouncing balls in the simulation.
/// It provides methods to create a new simulation, push a ball into the simulation,
/// and append multiple balls to the simulation.
/// 
/// In this simulation, radius and mass are equivalent, but we are using two fields for the code/logic comprehension.
pub struct BouncingBallSimulation {
    pub renderer_size: Vector2,
    pub balls: Vec<Ball>,
    pub velocity_gradient: Gradient,
    pub default_position: Vector2,
    pub default_velocity: Vector2,
    pub default_radius: f32,
    pub default_mass: f32,
    pub default_color: Color
}

impl BouncingBallSimulation {
    /// Creates a new `BouncingBallSimulation` instance with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `renderer` - The renderer used to render the simulation.
    /// * `velocity_gradient` - The gradient used to determine the velocity of the balls.
    /// * `default_position` - The default position of the balls. If `None`, the default position is `Vector2::zero()`.
    /// * `default_velocity` - The default velocity of the balls. If `None`, the default velocity is `Vector2::zero()`.
    /// * `default_radius` - The default radius of the balls. If `None`, the default radius is `15.0`.
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
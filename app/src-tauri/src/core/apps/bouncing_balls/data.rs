use serde::{Serialize, Deserialize};

use colorgrad::Color;

use crate::core::app_system::simulation::renderer::DataToSend;
use crate::core::sciences::maths::vector2::Vector2;

#[derive(Serialize, Clone)]
pub struct Ball {
    pub position: Vector2,
    pub velocity: Vector2,
    pub radius: f32,
    pub mass: f32,
    pub color: String, // store in hex format and not in colorgrad::Color to allow serialization
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

#[derive(Serialize, Clone)]
pub struct RendererData {
    pub balls: Vec<Ball>,
}

impl DataToSend for RendererData {}

#[derive(Deserialize, Clone)]
pub struct StarterData {
    pub positions: Vec<Vector2>
}
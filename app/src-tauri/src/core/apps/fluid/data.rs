use serde::{Serialize, Deserialize};

use crate::core::app_system::simulation::renderer::DataToSend;
use crate::core::sciences::maths::vector2::Vector2;

use super::main::FluidParticles;

#[derive(Serialize, Clone)]
pub struct RendererData {
    pub fluid_particles: FluidParticles,
}

impl DataToSend for RendererData {}

#[derive(Deserialize, Clone)]
pub struct StarterData {
    pub positions: Vec<Vector2>
}

#[derive(Deserialize)]
pub struct EventSettings {
    pub collision_restitution: f32,
    pub gravity: f32,
    pub target_density: f32,
    pub mass: f32,
    pub pressure_stiffness: f32,
    pub visual_filter: u8,
    pub smoothing_radius: f32,
    pub viscosity_strength: f32,
}
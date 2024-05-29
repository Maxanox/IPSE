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
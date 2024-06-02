use serde::{Serialize, Deserialize};
use crate::core::app_system::simulation::renderer::DataToSend;
use crate::core::sciences::maths::Vector2;

#[derive(Serialize, Clone)]
pub struct LightRigidBody {
    pub position: Vector2,
    pub rotation: f32,
    pub radius: f32,
    pub width: f32,
    pub height: f32,
    pub shape: bool,
}

#[derive(Serialize, Clone)]
pub struct RendererData {
    pub bodies: Vec<LightRigidBody>,
}

impl DataToSend for RendererData {}

#[derive(Deserialize, Clone)]
pub struct StarterData {}
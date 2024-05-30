use serde::{Serialize, Deserialize};
use crate::core::sciences::physics::rigid_body::r#struct::{RigidBody};

use crate::core::app_system::simulation::renderer::DataToSend;

#[derive(Serialize, Clone)]
pub struct RendererData {
    pub bodies: Vec<RigidBody>,
}

impl DataToSend for RendererData {}

#[derive(Deserialize, Clone)]
pub struct StarterData {}
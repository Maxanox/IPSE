use serde::{Serialize, Deserialize};
use crate::core::sciences::physics::rigid_body::r#struct::WorkSpace;
use crate::core::app_system::simulation::renderer::DataToSend;

#[derive(Serialize, Clone)]
pub struct RendererData {
    pub bodies: WorkSpace,
}

impl DataToSend for RendererData {}

#[derive(Deserialize, Clone)]
pub struct StarterData {}
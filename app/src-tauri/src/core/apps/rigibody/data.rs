use serde::{Serialize, Deserialize};

use crate::core::app_system::simulation::renderer::DataToSend;

#[derive(Serialize, Clone)]
pub struct RendererData {
    // TODO: Add the properties of the RendererData struct
}

impl DataToSend for RendererData {}

#[derive(Deserialize, Clone)]
pub struct StarterData {
    // TODO: Add the properties of the StarterData struct
}
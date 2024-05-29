//use crate::core::app_system::simulation::template::SimulationTemplate;
//use crate::core::app_system::simulation::renderer::DataToSend;

//use super::main::TA_SIMULATION;
//use super::data::*;

// Obligatory implementation of the `SimulationTemplate` trait for the `TA_SIMULATION` struct.
//impl SimulationTemplate for TA_SIMULATION {
//    fn initialize(&mut self, renderer_size: Vector2, serialized_data: Option<String>) -> Result<(), String> {
//        todo!()
//    }
//
//    fn next_step(&mut self, dt: f32) -> Result<(), String> {
//        todo!()
//    }
//
//    fn get_data_to_render(&self) -> Result<Box<dyn DataToSend>, String> {
//        todo!()
//    }
//}
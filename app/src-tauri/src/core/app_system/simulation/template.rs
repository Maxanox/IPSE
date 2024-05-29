use crate::core::sciences::maths::vector2::Vector2;

use super::renderer::DataToSend;


/// The `SimulationTemplate` trait is used by `SimulationManager` to run different
/// simulations.
/// 
/// This trait provides methods for initializing the simulation, performing the next step of the simulation 
/// and retrieving the data associated with the template.
/// 
/// Send is required to allow the trait to be used across threads.
/// 
pub trait SimulationTemplate: Send {

    /// Initializes the simulation with the given renderer size and starter data.
    ///
    /// # Arguments
    ///
    /// * `renderer_size` - The size of the renderer as a `Vector2`.
    /// * `serialized_data` - The starter data as a json `String`.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the initialization was successful, otherwise returns an error message as a `String`.
    fn initialize(&mut self, renderer_size: Vector2, serialized_data: Option<String>) -> Result<(), String>;

    /// Performs the next step of the simulation.
    ///
    /// # Arguments
    ///
    /// * `dt` - The time step for the simulation.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the step was successful, otherwise returns an error message as a `String`.
    fn next_step(&mut self, dt: f32) -> Result<(), String>;

    /// Retrieves the data associated with the template.
    ///
    /// # Returns
    ///
    /// Returns `Ok` with a boxed trait object implementing `RendererData` if the data retrieval was successful,
    /// otherwise returns an error message as a `String`.
    fn get_data_to_render(&self) -> Result<Box<dyn DataToSend>, String>;

    /// Handles an event for the simulation.
    /// 
    /// This method is called by the `SimulationManager` when an event is received.
    /// Usually, we match the event to a specific action in the simulation.
    ///
    /// # Arguments
    ///
    /// * `event` - The event as a `String`.
    /// * `data` - Additional data associated with the event as an optional json `String`.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the event was handled successfully, otherwise returns an error message as a `String`.
    fn event_handler(&mut self, event: String, data: Option<String>) -> Result<(), String>;
}
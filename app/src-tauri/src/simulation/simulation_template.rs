use crate::simulation::renderer::RendererData;
use crate::simulation::renderer::StarterData;
use crate::simulation::custom_maths::vector2::Vector2;


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
    /// * `data` - The starter data as a boxed trait object implementing `StarterData`.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the initialization was successful, otherwise returns an error message as a `String`.
    fn initialize(&mut self, renderer_size: Vector2, data: Option<Box<dyn StarterData>>) -> Result<(), String>;

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
    fn get_data_to_render(&self) -> Result<Box<dyn RendererData>, String>;
}
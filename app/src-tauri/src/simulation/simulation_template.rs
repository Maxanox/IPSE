use crate::simulation::renderer::RendererData;
use crate::simulation::custom_maths::vector2::Vector2;

/// A trait representing a simulation template.
/// 
/// The `SimulationTemplate` trait is used by the simulation manager to run different
/// simulations. It is responsible for performing the next step of the simulation 
/// and retrieving the data associated with the template.
/// 
/// Type `T` is the type of the data associated with the template. This data is used by the renderer to
/// render the simulation.
/// 
/// Send is required to allow the trait to be used across threads.
pub trait SimulationTemplate: Send {
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

    /// Sets the size of the renderer.
    ///
    /// # Arguments
    ///
    /// * `size` - The size of the renderer as a `Vector2`.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the size was set successfully, otherwise returns an error message as a `String`.
    fn set_renderer_size(&mut self, size: Vector2) -> Result<(), String>;

    /// Retrieves the data associated with the template.
    ///
    /// # Returns
    ///
    /// Returns `Ok` with a boxed trait object implementing `RendererData` if the data retrieval was successful,
    /// otherwise returns an error message as a `String`.
    fn get_renderer_data(&self) -> Result<Box<dyn RendererData>, String>;
}
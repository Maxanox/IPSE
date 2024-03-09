use crate::simulation::renderer::RendererData;

/// A trait representing a simulation template.
/// 
/// Send is required to allow the trait to be used across threads.
pub trait Template: Send {
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
    fn get_renderer_data(&self) -> Box<dyn RendererData>;
}
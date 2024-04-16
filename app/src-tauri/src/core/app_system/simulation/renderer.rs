use erased_serde::serialize_trait_object;

use crate::core::sciences::maths::vector2::Vector2;

/// A trait for data to send to the frontend, that can be serialized using erased_serde.
pub trait DataToSend: erased_serde::Serialize {}

serialize_trait_object!(DataToSend);

/// Represents a renderer for simulations.
pub struct Renderer {
    pub size: Vector2,
    pub window: tauri::Window
}

impl Renderer {
    /// Creates a new `Renderer` instance with the specified size and window.
    ///
    /// # Arguments
    ///
    /// * `size` - The size of the renderer.
    /// * `window` - The window to render on.
    ///
    /// # Returns
    ///
    /// A new `Renderer` instance.
    pub fn new(size: Vector2, window: tauri::Window) -> Self {
        Self {
            size,
            window
        }
    }

    /// Renders the given data using the renderer.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to render.
    ///
    /// # Returns
    ///
    /// An `Ok` result if the rendering was successful, otherwise an `Err` containing an error message.
    pub fn render(&self, data: Box<dyn DataToSend>) -> Result<(), String> {
        match self.window.emit("render", &data) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string())
        }
    }
}
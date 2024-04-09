use std::any::Any;

use erased_serde::serialize_trait_object;

use super::custom_maths::vector2::Vector2;

/// A trait for renderer data that can be serialized using erased_serde.
pub trait RendererData: erased_serde::Serialize {}

/// A trait for starter data that can be serialized using erased_serde.
pub trait StarterData: erased_serde::Serialize {
    /// Returns self as `&dyn Any`.
    /// 
    /// # Example
    /// 
    /// Usage in a function that takes a `Box<dyn StarterData>` as an argument.
    /// Like in the `SimulationTemplate::initialize` method.
    /// 
    /// ```
    /// fn initialize(data: Box<dyn StarterData>) -> Result<(), String> {
    ///     self.renderer_size = renderer_size;
    ///     let data = match data.as_any().downcast_ref::<Vec<Ball>>() {
    ///         Some(balls) => balls,
    ///         None => return Err("Invalid starter data type".to_string())
    ///     };
    /// 
    ///     Ok(())
    /// }
    /// ```
    fn as_any(&self) -> &dyn Any;
}

serialize_trait_object!(RendererData);
serialize_trait_object!(StarterData);

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
    pub fn render(&self, data: Box<dyn RendererData>) -> Result<(), String> {
        match self.window.emit("render", &data) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string())
        }
    }
}
use erased_serde::serialize_trait_object;

use super::custom_maths::vector2::Vector2;

pub trait RendererData: erased_serde::Serialize {}

serialize_trait_object!(RendererData);

/// Represents a renderer for simulations.
pub struct Renderer {
    pub size: Vector2,
    pub window: tauri::Window
}

impl Renderer {
    pub fn new(size: Vector2, window: tauri::Window) -> Self {
        Self {
            size,
            window
        }
    }

    pub fn render(&self, data: Box<dyn RendererData>) -> Result<(), String> {
        match self.window.emit("render", &data) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string())
        }
    }
}
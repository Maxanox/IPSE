use crate::custom_maths::vector2::Vector2;

/// Represents a renderer for simulations.
pub struct Renderer {
    pub width: f32,
    pub height: f32,
}

impl Renderer {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
        }
    }
}

pub trait RendererData {
    fn get_positions(&self) -> Vector2;
}
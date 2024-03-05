use serde::Serialize;

use crate::custom_maths::vector2::Vector2;

#[derive(Clone, Serialize)]
pub struct RenderPayload {
  pub positions: Vec<(Vector2, String)>,
}
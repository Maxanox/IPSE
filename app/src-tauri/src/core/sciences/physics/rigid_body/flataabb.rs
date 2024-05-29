use super::r#struct::{Vector2D, AABB};
use super::vectormath::c_vect;

#[allow(dead_code)]
pub fn init_aabb(min_s:Vector2D,max_s:Vector2D)->AABB{
    AABB{
        min:min_s,
        max:max_s,
    }
}

#[allow(dead_code)]
pub fn init_hard_aabb(min_x:f64,min_y:f64,max_x:f64,max_y:f64)->AABB{
    AABB{
        min : c_vect(min_x,min_y),
        max: c_vect(max_x,max_y),
    }
}
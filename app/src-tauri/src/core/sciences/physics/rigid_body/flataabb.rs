use crate::r#struct::{ShapeType, Vector2D,WorkSpace,FlatTransform,AABB};
use crate::r#struct::RigidBody;
use  crate::physics::G;
use std::f64::consts::PI;
use crate::flattransfom::init_tf_zero;
use crate::r#struct::ShapeType::Circle;
use crate::vectormath::{dot_s, sm, mn, div_s, c_vect, vec_zero, transform_v};

pub fn init_aabb(min_s:Vector2D,max_s:Vector2D)->AABB{
    AABB{
        min:min_s,
        max:max_s,
    }
}

pub fn init_hard_aabb(min_x:f64,min_y:f64,max_x:f64,max_y:f64)->AABB{
    AABB{
        min : c_vect(min_x,min_y),
        max: c_vect(max_x,max_y),
    }
}
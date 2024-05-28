use crate::r#struct::{ShapeType, Vector2D,WorkSpace,FlatTransform,RigidBody};
use std::f64::consts::PI;
use std::f64::consts;
use std::ptr::{null, null_mut};
use crate::r#struct::ShapeType::Circle;
use crate::vectormath::{dot_s, sm, mn, div_s, vec_zero};

pub fn cos_me(i:f64)->f64{
    f64::cos(i)
}
pub fn sin_me(i:f64)->f64{
    f64::sin(i)
}

impl FlatTransform{
    pub fn transform(&mut self,x:f64,y:f64,angle:f64){
        self.pos_x=x;
        self.pos_y=y;
        self.cos=cos_me(angle);
        self.sin=sin_me(angle);
    }
}
pub fn init_tf_zero()->FlatTransform{
    FlatTransform{
        pos_x:0.0,
        pos_y:0.0,
        cos:0.0,
        sin:0.0,
    }
}
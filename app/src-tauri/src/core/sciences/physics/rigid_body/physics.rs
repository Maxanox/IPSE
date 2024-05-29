use crate::r#struct::Vector2D;
use crate::r#struct::RigidBody;
use crate::vectormath::{div_s, dot_s, sm, mn};
use  crate::r#struct::ShapeType;

pub const G : Vector2D = Vector2D{x:0.0,y:-9.81,};

impl Vector2D{
    pub fn negate(&mut self ){
        Vector2D{x:self.x*-1.0,y:-self.y*-1.0};
    }
    pub fn isequal(&mut self, v: Vector2D)->bool{
        self.x==v.x && self.y==v.y
    }
}
impl RigidBody{
   pub fn add_force(&mut self,amount:Vector2D){
       self.force = amount;
   }
}
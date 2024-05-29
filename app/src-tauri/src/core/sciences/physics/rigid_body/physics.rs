use super::r#struct::Vector2D;
use super::r#struct::RigidBody;

pub const G : Vector2D = Vector2D{x:0.0,y:-9.81,};

impl Vector2D{
    #[allow(dead_code)]
    pub fn negate(&mut self ){
        Vector2D{x:self.x*-1.0,y:-self.y*-1.0};
    }

    #[allow(dead_code)]
    pub fn isequal(&mut self, v: Vector2D)->bool{
        self.x==v.x && self.y==v.y
    }
}
impl RigidBody{
    #[allow(dead_code)]
   pub fn add_force(&mut self,amount:Vector2D){
       self.force = amount;
   }
}
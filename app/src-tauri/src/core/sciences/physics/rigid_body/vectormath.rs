use crate::r#struct::{RigidBody, Vector2D};
use  crate::r#struct::FlatTransform;

const VERY_SMALL_AMOUNT: f64 = 0.0005;
pub fn c_vect(xx:f64,yy:f64)->Vector2D{
    Vector2D{x:xx,y:yy}
}
pub fn sm(v1: Vector2D, v2: Vector2D)->Vector2D{
    Vector2D{x:v1.x+v2.x,y:v1.y+v2.y,}
}

pub fn mn(v1: Vector2D, v2: Vector2D)->Vector2D{
    Vector2D{x:v1.x-v2.x,y:v1.y-v2.y,}
}

pub fn dot_s(v1: Vector2D, s : f64)->Vector2D{
    Vector2D{x:v1.x*s,y:v1.y*s}
}

pub fn div_s(v1: Vector2D, s : f64)->Vector2D{
    Vector2D{x:v1.x/s,y:v1.y*s}
}

pub fn vec_zero()->Vector2D{
    Vector2D{x:0.0,y:0.0,}
}

pub fn transform_v(v: Vector2D,tf:FlatTransform)->Vector2D{
    let rx = tf.cos * v.x - tf.sin*v.y;
    let ry = tf.sin *v.x + tf.cos *v.y;

    let tx = rx + tf.pos_x;
    let ty = ry + tf.pos_y;

    return Vector2D{x:tx,y:ty,}
}

impl Vector2D {

    pub fn get_x (self : &Self) -> f64{
        self.x
    }
    pub fn get_y (self : &Self) -> f64{
        self.y
    }

    pub fn set_x(self : &mut Self,x:f64){
        self.x = x;
    }

    pub fn set_y(self : &mut Self,y:f64){
        self.y = y;
    }

    pub fn len_sq(&self)->f64{
        self.x * self.x + self.y * self.y
    }
    pub fn dist_sq(&self,v:Vector2D)->f64{
        let dx = self.x - v.x;
        let dy = self.y - v.y;
        dx * dx + dy * dy
    }
    pub fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn dist(&self, v: Vector2D) -> f64 {
        let dx = self.x - v.x;
        let dy = self.y - v.y;
        (dx * dx + dy * dy).sqrt()
    }
    pub fn normalize(&self) -> Vector2D {
        let l = self.len();
        if l > 0.0 {
            Vector2D { x: self.x / l, y: self.y / l }
        } else {
            vec_zero()
        }
    }


    pub fn nearly_equal(&self,b:Vector2D)->bool{
        nearly_eq(self.x,b.x) && nearly_eq(self.y,b.y)
    }

    pub fn dot(&self, v: Vector2D) -> f64 {
        self.x * v.x + self.y * v.y
    }
    pub fn cross(&self, v: Vector2D) -> f64 {
        self.x * v.y - self.y * v.x
    }
}

pub fn nearly_eq(a:f64,b:f64)->bool{
    (a-b).abs() < VERY_SMALL_AMOUNT
}
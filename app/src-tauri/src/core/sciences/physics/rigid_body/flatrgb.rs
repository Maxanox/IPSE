use super::r#struct::{ShapeType, Vector2D,WorkSpace,FlatTransform,AABB};
use super::r#struct::RigidBody;
use  super::physics::G;
use super::flataabb::init_hard_aabb;
use std::f64::consts::PI;
use super::flattransfom::init_tf_zero;
use super::r#struct::ShapeType::Circle;
use super::vectormath::{dot_s, sm, c_vect, vec_zero, transform_v};


impl RigidBody{
    pub fn moves(&mut self,v:Vector2D){
        self.position = sm(self.position,v);
        self.tfv_required =true;
        self.aabb_update= true;
    }
    pub fn moves_to(&mut self,pos:Vector2D){
        self.position = pos;
        self.tfv_required =true;
        self.aabb_update= true;
    }
    #[allow(dead_code)]
    pub fn rotate_box(&mut self,amount:f64){
        self.angle +=amount;
        self.tfv_required=true;
        self.aabb_update= true;
    }
    pub fn get_tfv(&mut self)->Vec<Vector2D>{
        if self.tfv_required {
            let mut tf :FlatTransform = init_tf_zero();
            tf.transform(self.position.x,self.position.y,self.angle);
            for i in 0..self.vertices.len(){
                let v = self.vertices[i];
                self.transformed_vertices[i]=transform_v(v,tf);
            }
        }
        self.tfv_required = false;
        self.transformed_vertices.clone()
    }
    pub fn step_body(&mut self,mut time:f64,iterations:i32){
        if self.is_static { return; }
        //let acc = div_s(self.force,self.mass);
        //self.linear_velocity = sm(self.linear_velocity,dot_s(acc,time));
        time = time / iterations as f64;
        self.linear_velocity = sm(self.linear_velocity,dot_s(G,time));
        self.position = sm(self.position,dot_s(self.linear_velocity,time));
        self.angle += self.angular_velocity * time;

        self.force=vec_zero();
        self.tfv_required = true;
        self.aabb_update= true;
    }
    #[allow(dead_code)]
    pub fn get_aabb(&mut self)->AABB{
        if self.aabb_update {
            let mut min_x = f64::MAX;
            let mut min_y = f64::MAX;
            let mut max_x = f64::MIN;
            let mut max_y = f64::MIN;
            if which_shape(self.shape) == 1 {
                let vertices = self.get_tfv();
                for i in 0..vertices.len() {
                    let v = vertices[i];
                    if v.x < min_x { min_x = v.x; }
                    if v.x > max_x { max_x = v.x; }
                    if v.y < min_y { min_y = v.y; }
                    if v.y > max_x { max_y = v.y; }
                }
            } else {
                min_x = self.position.x - self.radius;
                min_y = self.position.y - self.radius;
                max_x = self.position.x + self.radius;
                max_y = self.position.y + self.radius;
            }
            self.aabb=init_hard_aabb(min_x,min_y,max_x,max_y);
        }
        self.aabb_update=false;
        return self.aabb
    }
}

#[allow(dead_code)]
pub fn creat_vertices_box(w:f64,h:f64)->Vec<Vector2D>{
    let left = w/2.0;
    let right = left + w;
    let bottom = h/2.0;
    let top = bottom+h;

    let mut fv:Vec<Vector2D> = vec![vec_zero();4];
    fv[0] = c_vect(left,top);
    fv[1] = c_vect(right,top);
    fv[2] = c_vect(right,bottom);
    fv[3] = c_vect(left,bottom);
    fv
}
pub fn which_shape(x : ShapeType)->u8{
    match x {
        ShapeType::Circle => 0,
        ShapeType::Box => 1,
    }
}

#[allow(dead_code)]
pub fn initializer_r(pos :Vector2D,d:f64,mas:f64,rest:f64,
                   are:f64,b:bool,rad:f64,w:f64,h:f64,sh:ShapeType)->RigidBody{
    let mut r = RigidBody {
        position:pos,
        linear_velocity:Vector2D{x:0.0,y:0.0,},
        angle:0.0,
        angular_velocity:0.0,
        force:vec_zero(),
        density:d,
        mass:mas,
        inv_mass: 0.0,
        restitution:rest,
        area:are,
        is_static:b,
        radius:rad,
        width:w,
        height:h,
        shape:sh,
        triangles:vec![0],
        vertices:vec![vec_zero()],
        transformed_vertices: vec![vec_zero()],
        aabb:AABB{min : vec_zero(),max:vec_zero()},
        tfv_required:false,
        aabb_update:true,
        index : -1,
    };
    if r.is_static { r.inv_mass= 1.0/r.mass; }
    if which_shape(sh)==1 {
        r.vertices = creat_vertices_box(r.width,r.height);
        r.triangles =triangulate_box();
        r.transformed_vertices=vec![vec_zero();r.vertices.len()];
    }
    r.tfv_required = false;
    r
}
#[allow(dead_code)]
pub fn clamp (v : f64, min : f64, max : f64) ->Result<f64,String>{
    if min==max { return Ok(min); }
    if min > max { return Err(format!("Minimum value ({}) cannot be greater than or equal to maximum value ({})", min, max)); }
    if v < min { return Ok(min); }
    if v > max { return Ok (max); }
    Ok(v)
}

pub fn clamp_for_int (v : i32, min : i32, max : i32) ->Result<i32,String>{
    if min==max { return Ok(min); }
    if min > max { return Err(format!("Minimum value ({}) cannot be greater than or equal to maximum value ({})", min, max)); }
    if v < min { return Ok(min); }
    if v > max { return Ok (max); }
    Ok(v)
}

pub fn triangulate_box()->Vec<i32>{
    let mut triangles = vec![0;6];
    triangles[0]=0;
    triangles[1]=1;
    triangles[2]=0;
    triangles[3]=0;
    triangles[4]=2;
    triangles[5]=3;
    triangles
}

#[allow(dead_code)]
pub fn create_circle_body(rad:f64,pos:Vector2D,d:f64,b:bool,rest:f64,world:WorkSpace)->RigidBody{
    let j = create_circle_body_check(rad,pos,d,b,rest,world);
    j.unwrap()

}
#[allow(dead_code)]
pub fn create_circle_body_check(rad:f64,pos:Vector2D,d:f64,b:bool,rest:f64,world : WorkSpace)->Option<RigidBody>{
    let area_a = rad*rad*PI;
    if area_a < world.mn_bs  { return None; }
    if area_a > world.mx_bs {return  None;}
    if d < world.mn_d { return  None;}
    if d > world.mx_d {return None;}
    let restit = clamp(rest,0.0,1.0).unwrap();
//mass = area*depth*density
    let mass = area_a*1.0*d;
    let body = initializer_r(pos,d,mass,restit,area_a,b,rad,0.0,0.0,Circle);
    Some(body)
    
}

#[allow(dead_code)]
pub fn create_box_body(width:f64,height : f64,pos:Vector2D,d:f64,b:bool,rest:f64,world:WorkSpace)->RigidBody{
    let j = super::flatrgb::create_box_body_check(width,height, pos, d, b, rest, world);
    j.unwrap()

}

pub fn create_box_body_check(width:f64,height:f64,pos:Vector2D,d:f64,b:bool,rest:f64,world : WorkSpace)->Option<RigidBody>{
    let area_a = width*height;
    if area_a < world.mn_bs  { return None; }
    if area_a > world.mx_bs {return  None;}
    if d < world.mn_d { return  None;}
    if d > world.mx_d {return None;}
    let restit = clamp(rest,0.0,1.0).unwrap();
//mass = area*depth*density
    let mass = area_a*d;
    let body = initializer_r(pos,d,mass,restit,area_a,b,0.0,width,height,ShapeType::Box);
    Some(body)

}


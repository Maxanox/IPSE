use serde::Serialize;
use crate::core::sciences::physics::rigid_body::flatrgb::initializer_r;
use crate::core::sciences::physics::rigid_body::r#struct::{RigidBody, ShapeType, Vector2D, WorkSpace};
use crate::core::sciences::physics::rigid_body::vectormath::c_vect;
//use super::data::*;


/// Represents a bouncing ball simulation.
///
/// This struct holds information about the bouncing balls in the simulation.
/// It provides methods to create a new simulation, push a ball into the simulation,
/// and append multiple balls to the simulation.
///
/// In this simulation, radius and mass are equivalent, but we are using two fields for the code/logic comprehension.

#[derive(Serialize,Debug,Clone)]
pub struct RigidSimulation{
    pub renderer_size:Vector2D,
    pub work_space: WorkSpace,
}

impl RigidSimulation {
    /// Creates a new `BouncingBallSimulation` instance with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `renderer` - The renderer used to render the simulation.
    /// * `default_workspace` -The space of work of the rigid bodies
    /// # Returns
    ///
    /// A new `RigidSimulation` instance.
    pub fn new(renderer_size: Vector2D, default_workspace:Option<WorkSpace> ) -> Self {
        let workspace = if let Some(workspace) = default_workspace{workspace} else {WorkSpace::new()};
        RigidSimulation{
            renderer_size,
            work_space:workspace,
        }
    }

    /// Creates a new body with the default position, either specific variable, either defaults and adds it to the simulation.
    pub fn add_default_circle(&mut self) {
        let mut ball = RigidBody::new_ball();
        ball.position=c_vect(self.renderer_size.x/2.0,self.renderer_size.y/2.0);
        self.push_body(ball);
    }

    pub fn add_default_box(&mut self) {
        let mut box_s = RigidBody::new_box();
        box_s.position=c_vect(self.renderer_size.x/2.0,self.renderer_size.y/2.0);
        self.push_body(box_s);
    }

    pub fn add_body(&mut self,density:f64,mass:f64,restitution:f64,
                      area:f64,is_static:bool,radius:f64,width:f64,height:f64,shape:ShapeType,inertia:f64) {
        let mut body = initializer_r(density,mass,restitution,area,is_static,radius,width,height,shape,inertia);
        body.position = c_vect(self.renderer_size.x/2.0,self.renderer_size.y/2.0);
        self.push_body(body);
    }


    /// Pushes a body into the simulation.
    ///
    /// # Arguments
    ///
    /// * `body` - The rigid body to be pushed into the simulation.
    pub fn push_body(&mut self, body: RigidBody) {
        self.work_space.add_body(body);
    }

    /// Appends multiple boxes to the simulation.
    ///
    /// # Arguments
    ///
    /// * `boxes` - A mutable reference to a vector of bodies to be appended to the simulation.
    pub fn append_bodies(&mut self, bodies: &mut Vec<RigidBody>) {
        self.work_space.body_list.append(bodies);
    }

    pub fn update(&mut self,dt:f64){
        let it = self.work_space.max_iter;
        self.work_space.step(dt,it);
    }
}


//use std::thread;
//use std::time::Duration;
//use std::f64::consts::PI;
//use std::cmp::Ordering;

use std::f64::consts::PI;
use crate::flatrgb::{creat_vertices_box, triangulate_box, which_shape};
use crate::vectormath::{c_vect, vec_zero};

#[derive(Debug, Clone, Copy)]
pub struct Vector2D {
    pub x : f64,
    pub y : f64,
}

#[derive(Debug, Clone, Copy)]
pub struct AABB {
    pub min: Vector2D,
    pub max: Vector2D,
}

#[derive(Debug, Clone, Copy)]
pub struct Particle2D{
    position : Vector2D,
    velocity : Vector2D,
    mass : f64,
}

#[derive(Debug, Clone, Copy)]
pub enum  ShapeType{
    Circle = 0,
    Box = 1
}

#[derive(Debug,Clone)]
pub struct RigidBody{
    pub position : Vector2D,
    pub linear_velocity : Vector2D,
    pub angle : f64,
    pub angular_velocity : f64,
    pub force : Vector2D,
    pub mass : f64,
    pub inv_mass:f64,
    pub density : f64,
    pub area : f64,
    pub restitution : f64,
    pub is_static : bool,
    pub radius : f64,
    pub width : f64,
    pub height:f64,
    pub shape:ShapeType,
    pub triangles : Vec<i32>,
    pub vertices: Vec<Vector2D>,
    pub transformed_vertices: Vec<Vector2D>,
    pub aabb : AABB,
    pub tfv_required:bool,
    pub aabb_update:bool,
    pub index: i32,

}

impl RigidBody {

    pub fn new_box()->RigidBody{
        let mut r = RigidBody {
            position:vec_zero(),
            linear_velocity:Vector2D{x:0.0,y:0.0,},
            angle:0.0,
            angular_velocity:0.0,
            force:vec_zero(),
            density:1.0,
            mass:5.0,
            inv_mass: 0.0,
            restitution:5.0,
            area:25.0,
            is_static:true,
            radius:0.0,
            width:5.0,
            height:5.0,
            shape:ShapeType::Box,
            triangles: triangulate_box(),
            vertices:vec![vec_zero()],
            transformed_vertices: vec![vec_zero()],
            aabb:AABB{min : vec_zero(),max:vec_zero()},
            tfv_required:false,
            aabb_update:true,
            index : -1,
        };
        r
    }

    pub fn new_ball()->RigidBody{
        let mut r = RigidBody {
            position:vec_zero(),
            linear_velocity:Vector2D{x:0.0,y:0.0,},
            angle:0.0,
            angular_velocity:0.0,
            force:vec_zero(),
            density:1.0,
            mass:5.0,
            inv_mass: 0.0,
            restitution:5.0,
            area:PI*25.0,
            is_static:true,
            radius:5.0,
            width:0.0,
            height:0.0,
            shape:ShapeType::Box,
            triangles: triangulate_box(),
            vertices:vec![vec_zero()],
            transformed_vertices: vec![vec_zero()],
            aabb:AABB{min : vec_zero(),max:vec_zero()},
            tfv_required:false,
            aabb_update:true,
            index : -1,
        };
        r
    }

    pub fn get_position(&self) -> Vector2D {
        self.position.clone()
    }

    pub fn set_position(&mut self, position: Vector2D) {
        self.position = position;
    }

    pub fn get_linear_velocity(&self) -> &Vector2D {
        &self.linear_velocity
    }

    pub fn set_linear_velocity(&mut self, linear_velocity: Vector2D) {
        self.linear_velocity = linear_velocity;
    }

    pub fn get_angle(&self) -> f64 {
        self.angle
    }

    pub fn set_angle(&mut self, angle: f64) {
        self.angle = angle;
    }

    pub fn get_angular_velocity(&self) -> f64 {
        self.angular_velocity
    }

    pub fn set_angular_velocity(&mut self, angular_velocity: f64) {
        self.angular_velocity = angular_velocity;
    }

    pub fn get_force(&self) -> &Vector2D {
        &self.force
    }

    pub fn set_force(&mut self, force: Vector2D) {
        self.force = force;
    }

    pub fn get_mass(&self) -> f64 {
        self.mass
    }

    pub fn set_mass(&mut self, mass: f64) {
        self.mass = mass;
    }

    pub fn get_inv_mass(&self) -> f64 {
        self.inv_mass
    }

    pub fn set_inv_mass(&mut self, inv_mass: f64) {
        self.inv_mass = inv_mass;
    }

    pub fn get_density(&self) -> f64 {
        self.density
    }

    pub fn set_density(&mut self, density: f64) {
        self.density = density;
    }

    pub fn get_area(&self) -> f64 {
        self.area
    }

    pub fn set_area(&mut self, area: f64) {
        self.area = area;
    }

    pub fn get_restitution(&self) -> f64 {
        self.restitution
    }

    pub fn set_restitution(&mut self, restitution: f64) {
        self.restitution = restitution;
    }

    pub fn get_is_static(&self) -> bool {
        self.is_static
    }

    pub fn set_is_static(&mut self, is_static: bool) {
        self.is_static = is_static;
    }

    pub fn get_radius(&self) -> f64 {
        self.radius.clone()
    }

    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }

    pub fn get_width(&self) -> f64 {
        self.width
    }

    pub fn set_width(&mut self, width: f64) {
        self.width = width;
    }

    pub fn get_height(&self) -> f64 {
        self.height
    }

    pub fn set_height(&mut self, height: f64) {
        self.height = height;
    }

    pub fn get_shape(&self) -> &ShapeType {
        &self.shape
    }

    pub fn set_shape(&mut self, shape: ShapeType) {
        self.shape = shape;
    }

    pub fn get_triangles(&self) -> &Vec<i32> {
        &self.triangles
    }

    pub fn set_triangles(&mut self, triangles: Vec<i32>) {
        self.triangles = triangles;
    }

    pub fn get_vertices(&self) -> &Vec<Vector2D> {
        &self.vertices
    }

    pub fn set_vertices(&mut self, vertices: Vec<Vector2D>) {
        self.vertices = vertices;
    }

    pub fn get_transformed_vertices(&self) -> &Vec<Vector2D> {
        &self.transformed_vertices
    }

    pub fn set_transformed_vertices(&mut self, transformed_vertices: Vec<Vector2D>) {
        self.transformed_vertices = transformed_vertices;
    }


    pub fn set_aabb(&mut self, aabb: AABB) {
        self.aabb = aabb;
    }

    pub fn get_tfv_required(&self) -> bool {
        self.tfv_required
    }

    pub fn set_tfv_required(&mut self, tfv_required: bool) {
        self.tfv_required = tfv_required;
    }

    pub fn get_aabb_update(&self) -> bool {
        self.aabb_update
    }

    pub fn set_aabb_update(&mut self, aabb_update: bool) {
        self.aabb_update = aabb_update;
    }

    pub fn get_index(&self) -> i32 {
        self.index
    }

    pub fn set_index(&mut self, index: i32) {
        self.index = index;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FlatTransform{
    pub pos_x:f64,
    pub pos_y:f64,
    pub sin:f64,
    pub cos:f64,
}

impl FlatTransform {
    pub fn get_pos_x(&self) -> f64 {
        self.pos_x
    }

    pub fn set_pos_x(&mut self, pos_x: f64) {
        self.pos_x = pos_x;
    }

    pub fn get_pos_y(&self) -> f64 {
        self.pos_y
    }

    pub fn set_pos_y(&mut self, pos_y: f64) {
        self.pos_y = pos_y;
    }

    pub fn get_sin(&self) -> f64 {
        self.sin
    }

    pub fn set_sin(&mut self, sin: f64) {
        self.sin = sin;
    }

    pub fn get_cos(&self) -> f64 {
        self.cos
    }

    pub fn set_cos(&mut self, cos: f64) {
        self.cos = cos;
    }
}


#[derive(Debug)]
pub struct WorkSpace<'a>{
    pub mn_bs : f64,
    pub mx_bs : f64,
    pub mn_d : f64,
    pub mx_d : f64,
    pub min_iter:i32,
    pub max_iter:i32,
    pub body_list:Vec<&'a mut RigidBody>,
    pub gravity: Vector2D,
    pub body_count : usize,
    pub contact_list : Vec<ManiFold>
}

impl <'a> WorkSpace<'a> {
    pub fn get_mn_bs(&self) -> f64 {
        self.mn_bs
    }

    pub fn set_mn_bs(&mut self, mn_bs: f64) {
        self.mn_bs = mn_bs;
    }

    pub fn get_mx_bs(&self) -> f64 {
        self.mx_bs
    }

    pub fn set_mx_bs(&mut self, mx_bs: f64) {
        self.mx_bs = mx_bs;
    }

    pub fn get_mn_d(&self) -> f64 {
        self.mn_d
    }

    pub fn set_mn_d(&mut self, mn_d: f64) {
        self.mn_d = mn_d;
    }

    pub fn get_mx_d(&self) -> f64 {
        self.mx_d
    }

    pub fn set_mx_d(&mut self, mx_d: f64) {
        self.mx_d = mx_d;
    }

    pub fn get_min_iter(&self) -> i32 {
        self.min_iter
    }

    pub fn set_min_iter(&mut self, min_iter: i32) {
        self.min_iter = min_iter;
    }

    pub fn get_max_iter(&self) -> i32 {
        self.max_iter
    }

    pub fn set_max_iter(&mut self, max_iter: i32) {
        self.max_iter = max_iter;
    }

    pub fn get_body_list(&self) -> &Vec<&'a mut RigidBody> {
        &self.body_list
    }

    pub fn set_body_list(&mut self, body_list:  Vec<&'a mut RigidBody>) {
        self.body_list = body_list;
    }

    pub fn get_body_in(&self,index:usize) -> &&mut RigidBody {
        &self.body_list[index]
    }

    pub fn get_gravity(&self) -> &Vector2D {
        &self.gravity
    }

    pub fn set_gravity(&mut self, gravity: Vector2D) {
        self.gravity = gravity;
    }

    pub fn get_body_count(&self) -> usize {
        self.body_count
    }

    pub fn set_body_count(&mut self, body_count: usize) {
        self.body_count = body_count;
    }

    pub fn get_contact_list(&self) -> &Vec<ManiFold> {
        &self.contact_list
    }

    pub fn set_contact_list(&mut self, contact_list: Vec<ManiFold>) {
        self.contact_list = contact_list;
    }
    pub fn new()->WorkSpace<'a> {
        WorkSpace {
            mn_bs: 0.01 * 0.01,
            mx_bs: 64.0 * 64.0,
            mn_d: 0.5,
            mx_d: 21.4,
            min_iter: 1,
            max_iter: 128,
            body_list: Vec::new(),
            gravity: c_vect(0.0, 9.81),
            body_count: 0,
            contact_list: Vec::new(),
        }
    }
}

#[derive(Debug,Clone)]
pub struct ManiFold {
    pub body_a: usize,
    pub body_b: usize,
    pub normal: Vector2D,
    pub depth: f64,
    pub contact1: Vector2D,
    pub contact2: Vector2D,
    pub contact_count: i32,
}

impl ManiFold {

    pub fn get_body_a(&self) -> usize {
        self.body_a.clone()
    }


    pub fn get_body_b(&self) -> usize{
        self.body_b.clone()
    }

    pub fn get_normal(&self) -> Vector2D {
        self.normal.clone()
    }

    pub fn set_normal(&mut self, normal: Vector2D) {
        self.normal = normal;
    }

    pub fn get_depth(&self) -> f64 {
        self.depth.clone()
    }

    pub fn set_depth(&mut self, depth: f64) {
        self.depth = depth;
    }

    pub fn get_contact1(&self) -> &Vector2D {
        &self.contact1
    }

    pub fn set_contact1(&mut self, contact1: Vector2D) {
        self.contact1 = contact1;
    }

    pub fn get_contact2(&self) -> &Vector2D {
        &self.contact2
    }

    pub fn set_contact2(&mut self, contact2: Vector2D) {
        self.contact2 = contact2;
    }

    pub fn get_contact_count(&self) -> i32 {
        self.contact_count
    }

    pub fn set_contact_count(&mut self, contact_count: i32) {
        self.contact_count = contact_count;
    }
}





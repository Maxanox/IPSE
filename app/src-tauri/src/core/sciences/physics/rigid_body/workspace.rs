use super::r#struct::{Vector2D, WorkSpace, ManiFold};
use super::r#struct::RigidBody;
use super::collisions::{intersect_aabbs,collide, find_contact_points, min};
use super::flatmanifold::init_manifold;
use super::flatrgb::clamp_for_int;
use super::vectormath::{dot_s, sm, mn, c_vect, vec_zero};

//static mut BODIES : Option<Vec<RigidBody>> = None;

impl <'a> WorkSpace<'a>{

    #[allow(dead_code)]
    pub fn broad_phase(&mut self){
        for i in 0..self.body_count {
            let body_a =  self.body_list[i].clone();
            let body_a_aabb = body_a.aabb.clone();


            for j in i + 1..self.body_count {
                let body_b = self.body_list[j].clone();
                let body_b_aabb = body_b.aabb.clone();

                if body_a.is_static && body_b.is_static { continue; }

                if !intersect_aabbs(body_a_aabb,body_b_aabb) {
                    continue;
                }
                self.contact_pair.push((i,j));

            }
        }
    }

    #[allow(dead_code)]
    pub fn narrow_phase(&mut self){
        let l =  self.contact_pair.len();
        for k in 0..l{
            let (i,j)=self.contact_pair[k];
            let (res, mut norm, mut depth) = collide(&mut self.body_list,i,j);
            if res {
                separate_bodies(&mut self.body_list,i,j,norm,depth);
                let (c1,c2,cn) = find_contact_points(&mut self.body_list,i,j);
                let  contact = init_manifold(i,j,norm,depth,c1,c2,cn);
                resolve_collision_with_rotation(&mut self.body_list,contact);
            }
        }
    }
    #[allow(dead_code)]
    pub fn add_body(&mut self, body: &'a mut RigidBody) {
        self.body_list.push(body);
        self.body_list[self.body_count].index = self.body_count as  i32;
        self.body_count+=1;
    }

    #[allow(dead_code)]
    pub fn remove_body(&mut self, index: usize) -> bool {
        if self.body_count <= index{
            return false
        }
        for i in index..self.body_count{
            self.body_list[i].index-=1;
        }
        self.body_count-=1;
        self.body_list.remove(index);
        true
    }

    #[allow(dead_code)]
    pub fn get_body(&mut self, id:usize)->(bool,&RigidBody){
        if id < self.body_list.len()  {
            return (true,&self.body_list[id]);
        }
        (false,&self.body_list[0])
    }
    #[allow(dead_code)]
    pub fn step_bodies(&mut self,time:f64,iterations:i32){
        for i in 0..self.body_list.len() {
            self.body_list[i].step_body(time,iterations);
        }
    }
    #[allow(dead_code)]
    pub fn step(&mut self,time:f64,mut iterations:i32){

        iterations = clamp_for_int(iterations,self.min_iter,self.max_iter).unwrap();
        for _ in 0..iterations {
            self.contact_pair = Vec::new();
            self.step_bodies(time,iterations);
            self.broad_phase();
            self.narrow_phase();
        }
    }
}




#[allow(dead_code)]
pub fn resolve_collision_basic(body_s:&mut Vec<& mut RigidBody>, mani: ManiFold){
    let idx_a = mani.get_body_a();
    let idx_b = mani.get_body_b();
    let normal = mani.get_normal();
    //let depth = mani.get_depth();
    let e = min(body_s[idx_a].restitution,body_s[idx_b].restitution);
    let rv :Vector2D = mn(body_s[idx_b].linear_velocity,body_s[idx_a].linear_velocity);
    if rv.dot(normal)>0.0 { return; }
    let mut j: f64 = -(1.0+e)*rv.dot(normal);
    let impulse= dot_s(normal,j);
    j = j/(body_s[idx_a].inv_mass + body_s[idx_b].inv_mass);
    body_s[idx_a].linear_velocity = mn(body_s[idx_a].linear_velocity,dot_s(impulse,body_s[idx_a].inv_mass));
    body_s[idx_b].linear_velocity = sm(body_s[idx_b].linear_velocity,dot_s(impulse,body_s[idx_b].inv_mass));
}

pub fn resolve_collision_with_rotation(body_s:&mut Vec<&mut RigidBody>,mani:ManiFold){
    let idx_a = mani.get_body_a();
    let idx_b = mani.get_body_b();

    let body_a_p= body_s[idx_a].get_position();
    let body_b_p = body_s[idx_b].get_position();

    let normal = mani.get_normal();
    let c1= mani.get_contact1();
    let c2 = mani.get_contact2();
    let cn = mani.get_contact_count();

    let restit_a=body_s[idx_a].get_restitution();
    let restit_b=body_s[idx_b].get_restitution();

    let body_a_inv_inertia=body_s[idx_a].get_inv_inertia();
    let body_b_inv_inertia=body_s[idx_b].get_inv_inertia();

    let e = min(restit_a,restit_b);
    let contact_list = vec![c1,c2];
    let mut each_impulses = vec![vec_zero();2];
    let mut ra_list = vec![vec_zero();2];
    let mut rb_list = vec![vec_zero();2];

    for i in 0..(cn as usize){
        let ra = mn(contact_list[i],body_a_p);
        let rb = mn(contact_list[i],body_b_p);

        ra_list[i]=ra;
        rb_list[i]=ra;

        let ra_perp = c_vect(ra.y*(-1.0),ra.x);
        let rb_perp = c_vect(rb.y*(-1.0),rb.x);

        let angular_linear_velocity_a=dot_s(ra_perp,body_s[idx_a].get_angular_velocity());
        let angular_linear_velocity_b=dot_s(rb_perp,body_s[idx_b].get_angular_velocity());

        let relative_velocity :Vector2D = mn(sm(body_s[idx_b].linear_velocity,angular_linear_velocity_b),
                                             sm(body_s[idx_a].linear_velocity,angular_linear_velocity_a));

        let contact_velocity_magnitude = relative_velocity.dot(normal) ;

        if contact_velocity_magnitude> 0.0{
            continue;
        }


        let ra_perp_dot_n = ra_perp.dot(normal);
        let rb_perp_dot_n = rb_perp.dot(normal);

        let denominator =
            body_s[idx_a].inv_mass + body_s[idx_b].inv_mass
            + (ra_perp_dot_n*ra_perp_dot_n)*body_s[idx_a].inv_inertia
            + (rb_perp_dot_n*rb_perp_dot_n)*body_s[idx_b].inv_inertia;

        let mut j: f64 = -(1.0+e)*relative_velocity.dot(normal);
        j/=denominator;
        j/=cn as f64;
        let impulse= dot_s(normal,j);
        each_impulses[i]=impulse;

        for k in 0..(cn as usize){
            let imp: Vector2D= each_impulses[k];
            body_s[idx_a].linear_velocity = sm(body_s[idx_a].linear_velocity,dot_s(dot_s(imp,-1.0),body_s[idx_a].inv_mass));
            body_s[idx_a].angular_velocity = (imp.cross(ra_list[k]))*(-1.0)*body_a_inv_inertia;
            body_s[idx_b].linear_velocity = sm(body_s[idx_b].linear_velocity,dot_s(imp,body_s[idx_b].inv_mass));
            body_s[idx_b].angular_velocity = (imp.cross(rb_list[k]))*body_b_inv_inertia;
        }
    }

}


pub fn separate_bodies(body_list:&mut Vec<& mut RigidBody>,idx_a:usize,idx_b:usize,norm:Vector2D,depth:f64){
    if body_list[idx_a].is_static {
        body_list[idx_b].moves(dot_s(norm, depth));
    } else if body_list[idx_b].is_static {
        body_list[idx_a].moves(dot_s(dot_s(norm, -1.0), depth));
    } else {
        body_list[idx_a].moves(dot_s(norm, depth / 2.0));
        body_list[idx_b].moves(dot_s(norm, depth / 2.0));
    }
}
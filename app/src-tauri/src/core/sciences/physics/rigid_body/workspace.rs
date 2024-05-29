use super::r#struct::{Vector2D, WorkSpace, ManiFold};
use super::r#struct::RigidBody;
use super::collisions::{intersect_aabbs,collide, find_contact_points, min};
use super::flatmanifold::init_manifold;
use super::flatrgb::clamp_for_int;
use super::vectormath::{dot_s, sm, mn};

//static mut BODIES : Option<Vec<RigidBody>> = None;

impl <'a> WorkSpace<'a>{
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
    pub fn step(&mut self,time:f64,mut iterations:i32){

        iterations = clamp_for_int(iterations,self.min_iter,self.max_iter).unwrap();
        for _ in 0..iterations {
            //Movement step
            for i in 0..self.body_list.len() {
                self.body_list[i].step_body(time,iterations);
            }
            self.contact_list = Vec::new();
            //Collision step
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

                    let (res, norm, depth) = collide(&mut self.body_list,i,j);
                    if res {
                        if body_a.is_static {
                            self.body_list[j].moves(dot_s(norm, depth));
                        } else if body_b.is_static {
                            self.body_list[i].moves(dot_s(dot_s(norm, -1.0), depth));
                        } else {
                            self.body_list[i].moves(dot_s(norm, depth / 2.0));
                            self.body_list[j].moves(dot_s(norm, depth / 2.0));
                        }
                        let (c1,c2,cn) = find_contact_points(&mut self.body_list,i,j);
                        let contact = init_manifold(i,j,norm,depth,c1,c2,cn);
                        self.contact_list.push(contact);
                    }
                }
            }
            let l =  self.contact_list.len();
            for i in 0..l{
                resolve_collision(&mut self.body_list,self.contact_list[i].clone())
            }
        }
    }
}

pub fn resolve_collision(body_s:&mut Vec<& mut RigidBody>, mani: ManiFold){
    let idx_a = mani.get_body_a();
    let idx_b = mani.get_body_b();
    let normal = mani.get_normal();
    //let depth = mani.get_depth();
    let e = min(body_s[idx_a].restitution,body_s[idx_b].restitution);
    let rv :Vector2D = mn(body_s[idx_b].linear_velocity,body_s[idx_a].linear_velocity);
    if rv.dot(normal)>0.0 { return; }
    let j: f64 = -(1.0+e)*rv.dot(normal);
    let impulse= dot_s(normal,j);
    //j = j/(body_s[idx_a].inv_mass + body_s[idx_b].inv_mass); unused variable
    body_s[idx_a].linear_velocity = mn(body_s[idx_a].linear_velocity,dot_s(impulse,body_s[idx_a].inv_mass));
    body_s[idx_b].linear_velocity = sm(body_s[idx_b].linear_velocity,dot_s(impulse,body_s[idx_b].inv_mass));
}



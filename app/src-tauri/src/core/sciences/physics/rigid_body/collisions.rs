use super::r#struct::{AABB, ShapeType, Vector2D};
use super::r#struct::RigidBody;
use super::flatrgb::which_shape;
use super::vectormath::{nearly_eq,dot_s, sm, mn, div_s, vec_zero,c_vect};

pub fn intersect_aabbs(a: AABB, b: AABB)->bool{
    if a.max.x <= b.min.x || b.max.x <= a.min.x ||a.max.y <= b.min.y || b.max.y <= a.min.y{
        return false;
    }
    true
}



pub fn collide(bodylist:&mut Vec<& mut RigidBody>,idx_a : usize, idx_b : usize)->(bool,Vector2D,f64){
    let mut normal : Vector2D = vec_zero();
    let mut depth :f64 =f64::MAX;
    let shape_a:ShapeType = bodylist[idx_a].shape;
    let shape_b:ShapeType = bodylist[idx_b].shape;

    let body_a_p =bodylist[idx_a].get_position();
    let body_b_p =bodylist[idx_b].get_position() ;

    let body_a_tfv =&mut bodylist[idx_a].get_tfv();
    let body_b_tfv =&mut bodylist[idx_a].get_tfv() ;

    if which_shape(shape_a)==1 {
        if which_shape(shape_b)==1 {
            return intersect_polygon2(body_a_p,body_a_tfv,body_b_p,body_b_tfv);
        }
        else if which_shape(shape_b)==0 {
            let (res,mut normal,depth)= intersect_polygon_circles2(body_b_p,bodylist[idx_b].get_radius(),body_a_p,body_a_tfv);
            normal = dot_s(normal,-1.0);
            return (res,normal,depth);
        }
    }
    else if which_shape(shape_a)==0 {

        if which_shape(shape_b)==1 {
            return intersect_polygon_circles2(body_a_p,bodylist[idx_a].get_radius(),body_b_p,body_b_tfv)
        }
        else if which_shape(shape_b)==0 {
            return intersect_circles(body_a_p,bodylist[idx_a].get_radius(),body_b_p,bodylist[idx_a].get_radius());
        }
    }
    (false,normal,depth)
}
#[allow(dead_code)]
pub fn intersect_polygon_circles2(center_c:Vector2D,c_rad:f64,polygon_center:Vector2D,vertices:&mut Vec<Vector2D>)->(bool,Vector2D,f64){
    let mut normal : Vector2D = vec_zero();
    let mut depth :f64 =f64::MAX;
    for i in 0..vertices.len(){
        let va = vertices[i];
        let vb = vertices[(i+1) % vertices.len()];
        let edge:Vector2D = mn(vb,va);
        let mut axis:Vector2D =c_vect(-edge.y,edge.x);
        axis =axis.normalize();
        let (min_a, max_a) = project_vertices(vertices,axis);
        let (min_b,max_b) = project_circle(center_c,c_rad,axis);
        if min_a >= max_b || min_b >= max_a {
            return (false,normal,depth);
        }
        let axis_depth = min(max_b-min_a,max_a-min_b);
        if axis_depth<depth {
            depth = axis_depth;
            normal = axis;
        }
    }
    let cp_index = closest_point(center_c,vertices);
    let cp = vertices[cp_index as usize];
    let axis = mn(cp,center_c);

    let (min_a, max_a) = project_vertices(vertices,axis);
    let (min_b,max_b) = project_circle(center_c,c_rad,axis);
    if min_a >= max_b || min_b >= max_a {
        return (false,normal,depth);
    }
    let axis_depth = min(max_b-min_a,max_a-min_b);
    if axis_depth<depth {
        depth = axis_depth;
        normal = axis;
    }
    let direction : Vector2D = mn(polygon_center,center_c);
    if direction.dot(normal)< 0.0 {
        normal = dot_s(normal,-1.0);
    }
    (true,normal,depth)
}

//Only polygon v2
#[allow(dead_code)]
pub fn intersect_polygon2(center_a:Vector2D,vec_a: &mut Vec<Vector2D>,center_b:Vector2D,vec_b:&mut Vec<Vector2D>)->(bool,Vector2D,f64){
    let mut normal : Vector2D = vec_zero();
    let mut depth :f64 =f64::MAX;
    for i in 0..vec_a.len(){
        let va = vec_a[i];
        let vb = vec_a[(i+1) % vec_a.len()];
        let edge:Vector2D = mn(vb,va);
        let mut axis:Vector2D =c_vect(-edge.y,edge.x);
        axis =axis.normalize();
        let (min_a, max_a) = project_vertices(vec_a,axis);
        let (min_b, max_b) = project_vertices(vec_b,axis);
        if min_a >= max_b || min_b >= max_a {
            return (false,normal,depth);
        }
        let axis_depth = min(max_b-min_a,max_a-min_b);
        if axis_depth<depth {
            depth = axis_depth;
            normal = axis;
        }
    }
    for j in 0..vec_b.len(){
        let va = vec_b[j];
        let vb = vec_b[(j+1) % vec_b.len()];
        let edge:Vector2D = mn(vb,va);
        let mut  axis:Vector2D =c_vect(-edge.y,edge.x);
        axis =axis.normalize();
        let (min_a, max_a) = project_vertices(vec_a,axis);
        let (min_b, max_b) = project_vertices(vec_b,axis);
        if min_a >= max_b || min_b >= max_a {
            return (false,normal,depth);
        }
        let axis_depth = min(max_b-min_a,max_a-min_b);
        if axis_depth<depth {
            depth = axis_depth;
            normal = axis;
        }
    }

    let direction : Vector2D = mn(center_b,center_a);
    if direction.dot(normal)< 0.0 {
        normal = dot_s(normal,-1.0);
    }
    (true,normal,depth)
}


//Intersect polygon circle 1
#[allow(dead_code)]
pub fn intersect_polygon_circles(center_c:Vector2D,c_rad:f64,vertices:&mut Vec<Vector2D>)->(bool,Vector2D,f64){
    let mut normal : Vector2D = vec_zero();
    let mut depth :f64 =f64::MAX;
    for i in 0..vertices.len(){
        let va = vertices[i];
        let vb = vertices[(i+1) % vertices.len()];
        let edge:Vector2D = mn(vb,va);
        let mut axis:Vector2D =c_vect(-edge.y,edge.x);
        axis =axis.normalize();
        let (min_a, max_a) = project_vertices(vertices,axis);
        let (min_b,max_b) = project_circle(center_c,c_rad,axis);
        if min_a >= max_b || min_b >= max_a {
            return (false,normal,depth);
        }
        let axis_depth = min(max_b-min_a,max_a-min_b);
        if axis_depth<depth {
            depth = axis_depth;
            normal = axis;
        }
    }
    let cp_index = closest_point(center_c,vertices);
    let cp = vertices[cp_index as usize];
    let axis = mn(cp,center_c);

    let (min_a, max_a) = project_vertices(vertices,axis);
    let (min_b,max_b) = project_circle(center_c,c_rad,axis);
    if min_a >= max_b || min_b >= max_a {
        return (false,normal,depth);
    }
    let axis_depth = min(max_b-min_a,max_a-min_b);
    if axis_depth<depth {
        depth = axis_depth;
        normal = axis;
    }

    let polygon_center = find_arithmetic_mean(&vertices);
    let direction : Vector2D = mn(polygon_center,center_c);
    if direction.dot(normal)< 0.0 {
        normal = dot_s(normal,-1.0);
    }
    (true,normal,depth)
}
//Closest point
#[allow(dead_code)]
pub fn closest_point(center_c:Vector2D,vertices:& Vec<Vector2D >)->i32{
    let mut result = 1;
    let mut min_dist:f64 = f64::MAX;
    for i in 0..vertices.len() as i32{
        let mut v = vertices[i as usize];
        let mut dist = v.dist(center_c);
        if dist<min_dist{
            min_dist=dist ;
            result = i ;
        }
    }
    result
}
//Only polygon 1
#[allow(dead_code)]
pub fn intersect_polygon(vec_a: &mut Vec<Vector2D>,vec_b:&mut Vec<Vector2D>)->(bool,Vector2D,f64){
    let mut normal : Vector2D = vec_zero();
    let mut depth :f64 =f64::MAX;
    for i in 0..vec_a.len(){
        let va = vec_a[i];
        let vb = vec_a[(i+1) % vec_a.len()];
        let edge:Vector2D = mn(vb,va);
        let mut axis:Vector2D =c_vect(-edge.y,edge.x);
        axis =axis.normalize();
        let (min_a, max_a) = project_vertices(vec_a,axis);
        let (min_b, max_b) = project_vertices(vec_b,axis);
        if min_a >= max_b || min_b >= max_a {
            return (false,normal,depth);
        }
        let axis_depth = min(max_b-min_a,max_a-min_b);
        if axis_depth<depth {
            depth = axis_depth;
            normal = axis;
        }
    }
    for j in 0..vec_b.len(){
        let va = vec_b[j];
        let vb = vec_b[(j+1) % vec_b.len()];
        let edge:Vector2D = mn(vb,va);
        let mut  axis:Vector2D =c_vect(-edge.y,edge.x);
        axis =axis.normalize();
        let (min_a, max_a) = project_vertices(vec_a,axis);
        let (min_b, max_b) = project_vertices(vec_b,axis);
        if min_a >= max_b || min_b >= max_a {
            return (false,normal,depth);
        }
        let axis_depth = min(max_b-min_a,max_a-min_b);
        if axis_depth<depth {
            depth = axis_depth;
            normal = axis;
        }
    }

    let center_a = find_arithmetic_mean(&vec_a);
    let center_b = find_arithmetic_mean(&vec_b);

    let direction : Vector2D = mn(center_b,center_a);
    if direction.dot(normal)< 0.0 {
        normal = dot_s(normal,-1.0);
    }
    (true,normal,depth)
}

pub fn find_arithmetic_mean(vertices:&Vec<Vector2D>) ->Vector2D{
    let mut sum_x= 0.0;
    let mut sum_y= 0.0;

    for i in 0..vertices.len(){
        let v = vertices[i];
        sum_x = sum_x +v.x;
        sum_y = sum_y +v.y;
    }
    c_vect(sum_x/vertices.len() as f64,sum_x/vertices.len() as f64)
}

pub fn min (a:f64,b:f64)->f64{
    if a < b {
        return a;
    }
    b
}

pub fn project_vertices(vertices:&mut Vec<Vector2D>,axis:Vector2D)->(f64,f64){
    let mut min : f64 = f64::MAX;
    let mut max : f64 = f64::MIN;
    for i in 0..vertices.len(){
        let v =  vertices[i];
        let proj = v.dot(axis);
        if proj < min {min = proj;}
        if proj > max {max = proj;}
    }
    (min,max)
}

pub fn project_circle (center:Vector2D,rad:f64,axis:Vector2D)->(f64,f64){
    //let mut min : f64 = f64::MAX;
    //let mut max : f64 = f64::MIN;
    let direction = axis.normalize();
    let direction_and_radius= div_s(direction,rad);
    let point1= sm(center,direction_and_radius);
    let point2 = mn(center,direction_and_radius);
    let mut min = point1.dot(axis);
    let mut max = point2.dot(axis);
    if (min>max){let tp = min;min = max; max = tp;}
    (min,max)



}

pub fn intersect_circles(center_a:Vector2D,radia_a:f64,center_b:Vector2D,radia_b:f64)->(bool,Vector2D,f64){
    let mut normal = vec_zero();
    let  distance = center_a.dist(center_b);
    let  radii = radia_a+radia_b;
    if(distance >= radii){
        return (false,normal,0.0);
    }
    normal = mn(center_b,center_a).normalize();
    let depth = radii - distance;
    return (true,normal,depth);
}

#[allow(dead_code)]
pub fn check_intersection_circles(bodies :&mut Vec<RigidBody>){
    for i in 0..bodies.len()-1{
        let mut rigid_a=&mut bodies[i].clone();
        for j in i+1..bodies.len(){
            let mut rigid_b=& mut bodies[j].clone();
            let (is_colliding, norm, depth) = intersect_circles(rigid_a.position, rigid_a.radius, rigid_b.position, rigid_b.radius);
            if is_colliding {
                rigid_a.moves(dot_s(norm,depth /2.0));
                rigid_b.moves_to(dot_s(norm,depth / 2.0));
            }
        }
    }
}

pub fn find_contact_points(body_s:&mut Vec<& mut RigidBody>,idx_a:usize,idx_b:usize)->(Vector2D,Vector2D,i32){
    let body_a_p =body_s[idx_a].get_position();
    let body_b_p =body_s[idx_b].get_position() ;

    let body_a_tfv =&mut body_s[idx_a].get_tfv();
    let body_b_tfv =&mut body_s[idx_a].get_tfv() ;

    let shape_a:ShapeType = body_s[idx_a].shape;
    let shape_b:ShapeType = body_s[idx_b].shape;
    let mut c1=vec_zero();
    let mut c2=vec_zero();
    let mut cn = 0;
    if which_shape(shape_a)==1 {
        if which_shape(shape_b)==1 {
            (c1,c2,cn)=find_contact_point_bb(body_a_tfv,body_b_tfv);
        }
        else if which_shape(shape_b)==0 {
            c1=find_contact_point_cb(body_b_p,body_s[idx_b].radius,body_a_p,body_a_tfv);
            cn = 1;
        }
    }
    else if which_shape(shape_a)==0 {

        if which_shape(shape_b)==1 {
            c1=find_contact_point_cb(body_a_p,body_s[idx_a].radius,body_b_p,body_b_tfv);
            cn = 1;
        }
        else if which_shape(shape_b)==0 {
            c1=find_contact_point_cc(body_a_p,body_s[idx_a].get_radius(),body_b_p);
            cn = 1;
        }
    }
    (c1,c2,cn)
}
pub fn find_contact_point_cb(center_c: Vector2D, rad_c:f64,center_b:Vector2D,vertices:&mut Vec<Vector2D>)->Vector2D{
    let l = vertices.len();
    let mut cp = vec_zero();
    let mut min_dsq = f64::MAX;
    for i in 0..l{
        let  va = vertices[i];
        let  vb = vertices[(i+1)%l];
        let  (mut dsq, c) =point_segment_distance(center_c,va,vb);
        if dsq < min_dsq {
            min_dsq = dsq;
            cp = c
        }
    }
    cp
}

pub fn point_segment_distance(p:Vector2D,a:Vector2D,b:Vector2D)->(f64,Vector2D){
    let ab = mn(b,a);
    let mut cp = vec_zero();
    let ap = mn(p,a);
    let proj = ap.dot(ab);
    let ab_len = ab.len_sq();
    let d = proj/ab_len;
    if d <=0.0  {
        cp = a;
    }
    else if d>=1.0 {
        cp = b;
    }
    else {
        dot_s(sm(a,ab),d);
    }
    (p.dist_sq(cp),cp)
}

pub fn find_contact_point_cc(center_a: Vector2D, rad_a:f64,center_b:Vector2D)->Vector2D{
    let mut ab = mn(center_a,center_b);
    let  dir = ab.normalize();
    dot_s(sm(center_a,dir),rad_a)
}

pub fn find_contact_point_bb(vertices_a:&mut Vec<Vector2D>,vertices_b:&mut Vec<Vector2D>)->(Vector2D,Vector2D,i32){
    let mut c1 = vec_zero();
    let mut c2 = vec_zero();
    let mut cn = 0;
    let mut min_dsq = f64::MAX;
    let l_a = vertices_a.len();
    let l_b = vertices_b.len();
    for i in 0.. l_a{
        let p = vertices_a[i];
        for j in 0..l_b{
            let va = vertices_b[j];
            let vb = vertices_b[(j+1) % l_b];
            let (dsq,cp) = point_segment_distance(p,va,vb);
            if nearly_eq(dsq,min_dsq) {
                if !cp.nearly_equal(c1) {
                    c2 = cp;
                    cn = 2;
                }
            }
            else if dsq < min_dsq {
                min_dsq = dsq;
                cn = 1;
                c1= cp;
            }
        }
    }
    for i in 0.. l_b{
        let p = vertices_b[i];
        for j in 0..l_a{
            let va = vertices_a[j];
            let vb = vertices_a[(j+1) % l_a];
            let (dsq,cp) = point_segment_distance(p,va,vb);
            if nearly_eq(dsq,min_dsq) {
                if !cp.nearly_equal(c1) {
                    c2 = cp;
                    cn = 2;
                }
            }
            else if dsq < min_dsq {
                min_dsq = dsq;
                cn = 1;
                c1= cp;
            }
        }
    }
    (c1,c2,cn)
}
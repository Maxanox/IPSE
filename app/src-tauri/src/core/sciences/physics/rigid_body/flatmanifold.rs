use super::r#struct::{Vector2D, ManiFold};

pub fn init_manifold(body_1:usize, body_2:usize, norm: Vector2D,
                         depths: f64, contact_a: Vector2D, contact_b: Vector2D,
                         nb_contact: i32) -> ManiFold {
    ManiFold {
        body_a: body_1,
        body_b: body_2,
        normal: norm,
        depth: depths,
        contact1: contact_a,
        contact2: contact_b,
        contact_count: nb_contact,
    }
}

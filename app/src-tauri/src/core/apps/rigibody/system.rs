use crate::core::sciences::physics::rigid_body::flatrgb::initializer_r;
use crate::core::sciences::physics::rigid_body::vectormath::c_vect;
use crate::core::app_system::simulation::template::SimulationTemplate;
use crate::core::app_system::simulation::renderer::DataToSend;
use crate::core::sciences::maths::Vector2;
use crate::core::sciences::physics::rigid_body::r#struct::ShapeType;

use super::main::RigidSimulation;
use super::data::*;

// Obligatory implementation of the `SimulationTemplate` trait for the `RigidSimulation` struct.
impl SimulationTemplate for RigidSimulation {
fn initialize(&mut self, renderer_size: Vector2, serialized_data: Option<String>) -> Result<(), String> {
    self.renderer_size = c_vect(renderer_size.x as f64, renderer_size.y as f64);

    let mut bord_gauche = initializer_r(5.0, 5.0, 0.0, self.renderer_size.y * 5.0, true, 0.0, 2.0, self.renderer_size.y, ShapeType::Box, 0.0);
    bord_gauche.position = c_vect(0.0, 0.0);

    let mut bord_bas = initializer_r(5.0, 5.0, 0.0, self.renderer_size.y * 5.0, true, 0.0, self.renderer_size.x, 2.0, ShapeType::Box, 0.0);
    bord_bas.position = c_vect(0.0, self.renderer_size.y - 2.5);

    let mut bord_haut = initializer_r(5.0, 5.0, 0.0, self.renderer_size.y * 2.0, true, 0.0, self.renderer_size.x, 2.0, ShapeType::Box, 0.0);
    bord_haut.position = c_vect(0.0, 0.0);

    let mut bord_d = initializer_r(5.0, 5.0, 0.0, self.renderer_size.y * 2.0, true, 0.0, 2.0, self.renderer_size.y, ShapeType::Box, 0.0);
    bord_d.position = c_vect(self.renderer_size.x - 2.5, 0.0);

    self.push_body(bord_haut);
    self.push_body(bord_bas);
    self.push_body(bord_gauche);
    self.push_body(bord_d);

    let radius = (renderer_size.x.min(renderer_size.y) / 2.0) - 10.0;
    let center = c_vect(renderer_size.x as f64 / 2.0, renderer_size.y as f64 / 2.0);

    for i in 0..10 {
        let angle = i as f64 * (2.0 * std::f64::consts::PI / 10.0);
        let x = center.x + radius as f64 * angle.cos();
        let y = center.y + radius as f64 * angle.sin();

        let mut body = initializer_r(5.0, 20.0, 0.2, 0.0, false, 0.0, 50.0, 50.0, ShapeType::Box, 0.0);
        body.position = c_vect(x, y);
        self.push_body(body);
    }

    Ok(())
}

    fn next_step(&mut self, dt: f32) -> Result<(), String> {
        self.update(dt as f64);
        Ok(())
    }

    fn event_handler(&mut self, event: String, data: Option<String>) -> Result<(), String> {
        unimplemented!();
    }

    fn get_data_to_render(&self) -> Result<Box<dyn DataToSend>, String> {
        let mut light_bodies: Vec<LightRigidBody> = Vec::new();

        for body in &self.work_space.body_list {
            let light_body = LightRigidBody {
                position: Vector2 {
                    x: body.position.x as f32,
                    y: body.position.y as f32
                },
                rotation: body.angle as f32,
                radius: body.radius as f32,
                width: body.width as f32,
                height: body.height as f32,
                shape: if let ShapeType::Box = body.shape { true } else { false }
            };

            light_bodies.push(light_body);
        }

        let data_to_render = RendererData {
            bodies: light_bodies
        };

        Ok(Box::new(data_to_render))
    }
}
use crate::custom_maths::vector2::Vector2;
use crate::physics::fluid::Fluid;
use crate::simulations::renderer::Renderer;

/// A struct representing a fluid simulation.
pub struct FluidSimulation {
    pub fluid: Fluid,
    pub running: bool,
    pub renderer: Renderer,
    last_update: std::time::Instant,
    dt: f32,
}
  
impl FluidSimulation {
    /// Creates a new `FluidSimulation` with the given particle mass and radius.
    ///
    /// # Arguments
    ///
    /// * `particle_mass` - The mass of each particle in the simulation.
    /// * `particle_radius` - The radius of each particle in the simulation.
    ///
    /// # Returns
    ///
    /// A new `FluidSimulation` instance.
    pub fn new(particle_mass: f32, particle_radius: f32, renderer_width: f32, renderer_height: f32) -> FluidSimulation {
        FluidSimulation {
            fluid: Fluid::new(particle_mass, particle_radius),
            running: false,
            renderer: Renderer {width: renderer_width, height: renderer_height},
            last_update: std::time::Instant::now(),
            dt: 0.0,
        }
    }

    /// Performs the next step in the fluid simulation.
    pub fn next_step(&mut self) {
        self.update_dt();
        self.apply_external_forces();
        self.resolve_collisions();
    }

    fn update_dt(&mut self) {
        self.dt = self.last_update.elapsed().as_secs_f32();
        self.last_update = std::time::Instant::now();
    }

    fn apply_external_forces(&mut self) {
        // Apply gravity
        for particle in &mut self.fluid.particles {
            particle.velocity += Vector2::down() * 9.81 * self.dt;
            particle.position += particle.velocity * self.dt;
        }
    }

    fn resolve_collisions(&mut self) {
        todo!("Implement resolve_collisions")
    }
}
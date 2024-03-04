use std::io::sink;

use crate::custom_maths::vector2::Vector2;
use crate::physics::fluid::Fluid;
use crate::simulations::renderer::Renderer;
use crate::custom_maths::utils::sign_f32;

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
            renderer: Renderer::new(renderer_width, renderer_height),
            last_update: std::time::Instant::now(),
            dt: 0.0,
        }
    }

    /// Updates the time delta for the simulation.
    pub fn update_dt(&mut self) {
        self.dt = self.last_update.elapsed().as_secs_f32() * 5.0;
        self.last_update = std::time::Instant::now();
    }

    /// Performs the next step in the fluid simulation.
    pub fn next_step(&mut self) -> Vec<Vector2> {
        self.apply_external_forces();
        self.resolve_collisions();

        for particle in &mut self.fluid.particles {
            particle.position += particle.velocity * self.dt;
        }

        // Optimisation à revoir, mais génère un vecteur de positions à chaque frame
        self.fluid.get_particle_positions()
    }

    /// Apply external forces to the fluid particles.
    fn apply_external_forces(&mut self) {
        // Apply gravity
        for particle in &mut self.fluid.particles {
            particle.velocity += Vector2::down() * 9.81 * self.dt;
        }
    }

    fn resolve_collisions(&mut self) {
        for particle in &mut self.fluid.particles {
            if particle.position.x - self.fluid.particle_radius < 0.0 {
                particle.position.x = self.fluid.particle_radius;
                particle.velocity.x *= -1.0;
            } else if particle.position.x + self.fluid.particle_radius > self.renderer.width {
                particle.position.x = self.renderer.width - self.fluid.particle_radius;
                particle.velocity.x *= -1.0;
            }
            if particle.position.y - self.fluid.particle_radius < 0.0 {
                particle.position.y = self.fluid.particle_radius;
                particle.velocity.y *= -1.0;
            } else if particle.position.y + self.fluid.particle_radius > self.renderer.height {
                particle.position.y = self.renderer.height - self.fluid.particle_radius;
                particle.velocity.y *= -1.0;
            }
        }
    }
}
use crate::custom_maths::vector2::Vector2;

/// Represents a fluid particle in a simulation.
pub struct FluidParticle {
    pub position: Vector2,
    pub velocity: Vector2,
    pub acceleration: Vector2,
    // pub color: [f32; 4],
}

impl FluidParticle {
    /// Creates a new `FluidParticle` with the given x and y coordinates.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate of the particle's position.
    /// * `y` - The y coordinate of the particle's position.
    ///
    /// # Returns
    ///
    /// A new `FluidParticle` with the specified position and default velocity and acceleration.
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vector2::new(x, y),
            velocity: Vector2::zero(),
            acceleration: Vector2::zero(),
            // color: [0.0, 0.0, 0.0, 1.0],
        }
    }
}

/// Represents a fluid in a simulation.
pub struct Fluid {
    pub particles: Vec<FluidParticle>,
    pub particle_mass: f32,
    pub particle_radius: f32,
}

impl Fluid {
    /// Creates a new instance of `Fluid` with the given particle mass and radius.
    ///
    /// # Arguments
    ///
    /// * `particle_mass` - The mass of each particle in the fluid.
    /// * `particle_radius` - The radius of each particle in the fluid.
    ///
    /// # Returns
    ///
    /// A new instance of `Fluid`.
    pub fn new(particle_mass: f32, particle_radius: f32) -> Self {
        Self {
            particles: Vec::new(),
            particle_mass,
            particle_radius,
        }
    }

    /// Pushes a new particle into the fluid.
    ///
    /// # Arguments
    ///
    /// * `particle` - The `FluidParticle` to be added to the fluid.
    pub fn push_particle(&mut self, particle: FluidParticle) {
        self.particles.push(particle);
    }

    /// Returns the total mass of the fluid.
    ///
    /// # Returns
    ///
    /// The total mass of the fluid.
    pub fn get_mass(&self) -> f32 {
        self.particle_mass * self.particles.len() as f32
    }

    /// Returns the positions of all particles in the fluid.
    ///
    /// # Returns
    ///
    /// A vector containing the positions of all particles in the fluid.
    pub fn get_particle_positions(&self) -> Vec<Vector2> {
        self.particles.iter().map(|particle| particle.position).collect()
    }
}
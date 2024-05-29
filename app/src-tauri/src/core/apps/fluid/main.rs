use serde::Serialize;

use colorgrad::Gradient;
//use rayon::prelude::*;

use crate::core::sciences::maths::Vector2;

#[derive(Serialize, PartialEq, Clone, Debug)]
pub struct FluidParticles {
    // ALL PARTICLES PROPERTIES
    pub mass: f32,
    pub radius: f32,
    pub target_density: f32, 
    pub pressure_multiplier: f32,
    pub smoothing_radius: f32,
    // EACH PARTICLE PROPERTIES
    pub positions: Vec<Vector2>,
    pub predicted_positions: Vec<Vector2>,
    pub velocities: Vec<Vector2>,
    pub densities: Vec<f32>,
    pub colors: Vec<String>, // store in hex string format and not in colorgrad::Color to allow serialization
}

impl FluidParticles {
    pub fn new(mass: f32, radius: f32, target_density: f32, pressure_multiplier: f32, smoothing_radius: f32) -> Self {
        Self {
            // ALL PARTICLES PROPERTIES
            mass,
            radius,
            target_density,
            pressure_multiplier,
            smoothing_radius,
            // EACH PARTICLE PROPERTIES
            positions: Vec::new(),
            predicted_positions: Vec::new(),
            velocities: Vec::new(),
            densities: Vec::new(),
            colors: Vec::new()
        }
    }

    pub fn calculate_pressure(&self, density: f32) -> f32 {
        let density_error = density - self.target_density;
        let pressure = density_error * self.pressure_multiplier;
        pressure        
    }

    pub fn len(&self) -> usize {
        self.positions.len()
    }

    pub fn push(&mut self, position: Vector2) -> () {
        self.positions.push(position);
        self.predicted_positions.push(position);
        self.velocities.push(Vector2::zero());
        self.densities.push(0.0);
        self.colors.push("#FFFFFFFF".to_string());
    }
}

pub struct Fluid {
    // FLUID PROPERTIES
    pub particles: FluidParticles,
    pub gravity: f32,
    pub visual_filter: u8,
    pub collision_restitution: f32,
    mass: f32,
    // BOUNDARY PROPERTIES
    pub box_bound_x: f32,
    pub box_bound_y: f32,
    // OTHER PROPERTIES
    pub velocity_gradient: Gradient
}

impl Fluid {
    pub fn new(velocity_gradient: Gradient) -> Self {
        //let particles = FluidParticles::new(0.0, 5.0, 0.5, 3.0, 30.0); # diffusion gazeuse
        let particles = FluidParticles::new(0.0, 5.0, 0.75, 3.5, 30.0);
        let mut fluid = Fluid {
            // FLUID PROPERTIES
            particles,
            gravity: 0.0,
            visual_filter: 0,
            collision_restitution: 0.95,
            mass: 50.0,
            // BOUNDARY PROPERTIES
            box_bound_x: 800.0,
            box_bound_y: 600.0,
            // OTHER PROPERTIES
            velocity_gradient
        };

        fluid.update_particles_mass();

        fluid
    }

    pub fn update_particles_mass(&mut self) -> () {
        self.particles.mass = self.mass / self.particles.len() as f32;
    }

    #[allow(dead_code)]
    pub fn set_mass(&mut self, mass: f32) -> () {
        self.mass = mass;
        self.update_particles_mass();
    }

    #[allow(dead_code)]
    pub fn get_mass(&self) -> f32 {
        self.mass
    }

    fn smoothing_kernel(&self, distance: f32) -> f32 {
        if distance >= self.particles.smoothing_radius {
            return 0.0;
        }

        let volume = std::f32::consts::PI * self.particles.smoothing_radius.powf(4.0) / 6.0;
        //let value = f32::max(0.0, self.particles.smoothing_radius * self.particles.smoothing_radius - distance * distance);

        (self.particles.smoothing_radius - distance) * (self.particles.smoothing_radius - distance) / volume
    }

    fn calculate_density(&self, i: usize) -> f32 {
        let mut density = 0.0;
        
        // loop over all particles
        for other_position in self.particles.predicted_positions.iter() {
            let distance = self.particles.predicted_positions[i].distance_to(*other_position);
            let influence = self.smoothing_kernel(distance);
            density += self.particles.mass * influence;
        }

        density
    }

    fn smoothing_kernel_derivative(&self, distance: f32) -> f32 {
        if distance >= self.particles.smoothing_radius {
            return 0.0;
        }

        //let offset = self.particles.smoothing_radius * self.particles.smoothing_radius - distance * distance;
        let scale = 12.0 / (std::f32::consts::PI * self.particles.smoothing_radius.powf(4.0));

        (distance - self.particles.smoothing_radius) * scale
    }

    pub fn calculate_shared_pressure(&self, i1: usize, i2: usize) -> f32 {
        let density1 = self.particles.densities[i1];
        let density2 = self.particles.densities[i2];
        let presure1 = self.particles.calculate_pressure(density1);
        let presure2 = self.particles.calculate_pressure(density2);
        (presure1 + presure2) / 2.0
    }

    pub fn calculate_pressure_force(&self, i: usize) -> Vector2 {
        let mut pressure_force = Vector2::zero();

        for other_i in 0..self.particles.len() {
            if i == other_i {
                continue;
            }

            let offset = self.particles.predicted_positions[i] - self.particles.predicted_positions[other_i];
            let distance = offset.magnitude();
            let direction = if distance == 0.0 { Vector2::random() } else { offset / distance };
            let slope = self.smoothing_kernel_derivative(distance);
            let density = self.particles.densities[other_i];
            let shared_pressure = self.calculate_shared_pressure(i, other_i);

            pressure_force += direction * shared_pressure * slope * self.particles.mass / density;
        }

        pressure_force
    }

    pub fn resolve_collision(&mut self, i: usize) {
        let mut position = self.particles.positions[i];
        let mut velocity = self.particles.velocities[i];

        let coefficient_of_restitution = 0.95;

        if position.x - self.particles.radius < 0.0 {
            position.x = self.particles.radius;
            velocity.x = -velocity.x * coefficient_of_restitution;
        } else if position.x + self.particles.radius > self.box_bound_x {
            position.x = self.box_bound_x - self.particles.radius;
            velocity.x = -velocity.x * coefficient_of_restitution;
        }

        if position.y - self.particles.radius < 0.0 {
            position.y = self.particles.radius;
            velocity.y = -velocity.y * coefficient_of_restitution;
        } else if position.y + self.particles.radius > self.box_bound_y {
            position.y = self.box_bound_y - self.particles.radius;
            velocity.y = -velocity.y * coefficient_of_restitution;
        }

        self.particles.positions[i] = position;
        self.particles.velocities[i] = velocity;
    }

    pub fn update(&mut self, dt: f32) -> () {
        //let dt = dt * 1.5;

        // Apply gravity and predicted positions
        (0..self.particles.len()).for_each(|i| {
            self.particles.velocities[i] += Vector2::down() * self.gravity * dt;
            self.particles.predicted_positions[i] = self.particles.positions[i] + self.particles.velocities[i] * dt;
        });

        if self.visual_filter == 3 {
            // Calculate densities
            (0..self.particles.len()).for_each(|i| {
                self.particles.densities[i] = self.calculate_density(i);
                self.particles.colors[i] = self.velocity_gradient.at((self.particles.densities[i] * 1000.0 / self.particles.target_density) as f64).to_hex_string();
            });
        } else {
            // Calculate densities
            (0..self.particles.len()).for_each(|i| {
                self.particles.densities[i] = self.calculate_density(i);
            });
        }

        if self.visual_filter == 1 {
            // Calculate and apply pressure forces
            (0..self.particles.len()).for_each(|i| {
                let pressure_force = self.calculate_pressure_force(i);
                assert!(self.particles.densities[i] != 0.0, "density should not be zero");
                let pressure_acceleration = pressure_force / self.particles.densities[i];
                self.particles.velocities[i] += pressure_acceleration * dt;
                self.particles.colors[i] = self.velocity_gradient.at((self.particles.velocities[i].magnitude() / 100.0) as f64).to_hex_string();
            });
        } else if self.visual_filter == 2 {
            // Calculate and apply pressure forces
            (0..self.particles.len()).for_each(|i| {
                let pressure_force = self.calculate_pressure_force(i);
                self.particles.colors[i] = self.velocity_gradient.at((pressure_force.magnitude() * 100.0) as f64).to_hex_string();
                assert!(self.particles.densities[i] != 0.0, "density should not be zero");
                let pressure_acceleration = pressure_force / self.particles.densities[i];
                self.particles.velocities[i] += pressure_acceleration * dt;
            });
        }
        else {
            // Calculate and apply pressure forces
            (0..self.particles.len()).for_each(|i| {
                let pressure_force = self.calculate_pressure_force(i);
                assert!(self.particles.densities[i] != 0.0, "density should not be zero");
                let pressure_acceleration = pressure_force / self.particles.densities[i];
                self.particles.velocities[i] += pressure_acceleration * dt;
            });
        }

        // Update positions and resolve collisions
        (0..self.particles.len()).for_each(|i| {
            self.particles.positions[i] += self.particles.velocities[i] * dt;
            self.resolve_collision(i);
        });

        //dbg!(&self.particles);
    }
}
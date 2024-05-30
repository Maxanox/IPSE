use serde::Serialize;

use colorgrad::Gradient;
use serde_json::map::Iter;
//use rayon::prelude::*;

use crate::core::sciences::maths::Vector2;

const CELL_OFFSETS: [Vector2; 9] = [
    Vector2 { x: -1.0, y: -1.0 },
    Vector2 { x: 0.0, y: -1.0 },
    Vector2 { x: 1.0, y: -1.0 },
    Vector2 { x: -1.0, y: 0.0 },
    Vector2 { x: 0.0, y: 0.0 },
    Vector2 { x: 1.0, y: 0.0 },
    Vector2 { x: -1.0, y: 1.0 },
    Vector2 { x: 0.0, y: 1.0 },
    Vector2 { x: 1.0, y: 1.0 }
];

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
    pub spatial_lookup: Vec<(usize, usize)>,
    pub lookup_start: Vec<usize>,
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
            colors: Vec::new(),
            spatial_lookup: Vec::new(),
            lookup_start: Vec::new(),
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
        self.spatial_lookup.push((0, 0));
        self.lookup_start.push(0);
    }

    pub fn get_cell_position(&self, position: &Vector2) -> Vector2 {
        let x = (position.x / self.smoothing_radius).floor();
        let y = (position.y / self.smoothing_radius).floor();
        Vector2::new(x, y)
    }

    pub fn get_cell_key(&self, cell_position: &Vector2) -> usize {
        const HK1: usize = 15823;
        const HK2: usize = 9737333;
        let hash = cell_position.x as usize * HK1 + cell_position.y as usize * HK2;
        hash % self.spatial_lookup.len()
    }

    pub fn update_spatial_lookup(&mut self) {
        for (i, position) in self.positions.iter().enumerate() {
            // get_cell_position
            let cell_position = self.get_cell_position(position);
            
            // get_cell_key
            let cell_key = self.get_cell_key(&cell_position);

            self.spatial_lookup[i] = (i, cell_key);
            self.lookup_start[cell_key] = usize::MAX;
        }

        self.spatial_lookup.sort_unstable_by_key(|(_, key)| *key);

        for (i, (_, key)) in self.spatial_lookup.iter().enumerate() {
            let prev_key = if i == 0 { usize::MAX } else { self.spatial_lookup[i - 1].1 };
            if *key != prev_key {
                self.lookup_start[*key] = i;
            }
        }
    }

    pub fn for_each_particle_whitin_radius(&self, position: &Vector2) ->  () {
        let cell_position = self.get_cell_position(position);
        let sqr_radius = self.smoothing_radius * self.smoothing_radius;

        for offset in CELL_OFFSETS.iter() {
            let neighbor_position = cell_position + *offset;
            let neighbor_key = self.get_cell_key(&neighbor_position);

            let start_index = self.lookup_start[neighbor_key];

            for (i, key) in self.spatial_lookup.iter().skip(start_index) {
                if *key != neighbor_key {
                    break;
                }

                let sqr_distance = self.positions[*i].distance_to_squared(*position);

                if sqr_distance < sqr_radius {
                    // do something
                }
            }
        }
    }
}

pub struct Fluid {
    // FLUID PROPERTIES
    pub particles: FluidParticles,
    pub gravity: f32,
    pub visual_filter: u8,
    pub collision_restitution: f32,
    // BOUNDARY PROPERTIES
    pub box_bound_x: f32,
    pub box_bound_y: f32,
    // OTHER PROPERTIES
    pub velocity_gradient: Gradient
}

impl Fluid {
    pub fn new(velocity_gradient: Gradient) -> Self {
        //let particles = FluidParticles::new(0.0, 5.0, 0.5, 3.0, 30.0); # diffusion gazeuse
        let particles = FluidParticles::new(1.0, 5.0, 0.75, 3.5, 30.0);
        Fluid {
            // FLUID PROPERTIES
            particles,
            gravity: 0.0,
            visual_filter: 0,
            collision_restitution: 0.95,
            // BOUNDARY PROPERTIES
            box_bound_x: 800.0,
            box_bound_y: 600.0,
            // OTHER PROPERTIES
            velocity_gradient
        }
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
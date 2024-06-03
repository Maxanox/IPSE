use std::{cell, f32::consts::PI, mem::discriminant};

use serde::Serialize;

use colorgrad::Gradient;

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

    pub fn get_cell_hash(&self, cell_position: &Vector2) -> usize {
        const HK1: usize = 15823;
        const HK2: usize = 9737333;
        //dbg!(cell_position.x, cell_position.y, cell_position.x as usize, cell_position.y as usize);
        (cell_position.x * HK1 as f32 + cell_position.y * HK2 as f32) as usize
    }

    pub fn get_cell_key_from_hash(&self, hash: usize) -> usize {
        hash % self.spatial_lookup.len()
    }

    pub fn update_spatial_lookup(&mut self) {
        for (i, position) in self.positions.iter().enumerate() {
            // get_cell_position
            let cell_position = self.get_cell_position(position);

            // get_cell_hash
            let cell_hash = self.get_cell_hash(&cell_position);
            
            // get_cell_key_from_hash
            let cell_key = self.get_cell_key_from_hash(cell_hash);

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
}

pub struct Fluid {
    // FLUID PROPERTIES
    pub particles: FluidParticles,
    pub gravity: f32,
    pub visual_filter: u8,
    pub collision_restitution: f32,
    pub viscosity_strength: f32,
    pub interactive_force: bool,
    pub interactive_force_position: Vector2,
    pub interactive_force_mode: bool,
    // BOUNDARY PROPERTIES
    pub box_bound_x: f32,
    pub box_bound_y: f32,
    // OTHER PROPERTIES
    pub velocity_gradient: Gradient,
    pub density_gradient: Gradient
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
            viscosity_strength: 0.1,
            interactive_force: false,
            interactive_force_position: Vector2::zero(),
            interactive_force_mode: true,
            // BOUNDARY PROPERTIES
            box_bound_x: 800.0,
            box_bound_y: 600.0,
            // OTHER PROPERTIES
            velocity_gradient,
            density_gradient: colorgrad::CustomGradient::new().html_colors(&["#0077ff", "#ffffff", "ff3131"]).domain(&[0.0, 0.5, 1.0]).build().unwrap()
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
        let mut density = self.smoothing_kernel(0.0); // au moins la densité de la particule i


        // OPTIMIZATION: only search for neighbors in the cells grid
        for other_i in self.in_radius_neighbors_search(i) {
            let distance = self.particles.predicted_positions[i].distance_to(self.particles.predicted_positions[other_i]);
            let influence = self.smoothing_kernel(distance);
            density += self.particles.mass * influence;
        }

        // NO OPTIMIZATION: search for all neighbors
        //for other_i in 0..self.particles.len() {
        //    let distance = self.particles.predicted_positions[i].distance_to(self.particles.predicted_positions[other_i]);
        //    let influence = self.smoothing_kernel(distance);
        //    density += self.particles.mass * influence;
        //}

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

    fn in_radius_neighbors_search(&self, i: usize) -> Vec<usize> {
        let mut neighbors = Vec::new();
        
        let cell_position = self.particles.get_cell_position(&self.particles.predicted_positions[i]);
        let sqr_radius = self.particles.smoothing_radius * self.particles.smoothing_radius;

        for offset in CELL_OFFSETS.iter() {
            let neighbor_position = cell_position + *offset;
            let neighbor_hash = self.particles.get_cell_hash(&neighbor_position);
            let neighbor_key = self.particles.get_cell_key_from_hash(neighbor_hash);

            let start_index = self.particles.lookup_start[neighbor_key];

            for (other_i, key) in self.particles.spatial_lookup.iter().skip(start_index) {
                if *key != neighbor_key {
                    break;
                }

                let position = self.particles.get_cell_position(&self.particles.predicted_positions[*other_i]);
                let hash = self.particles.get_cell_hash(&position);

                if hash != neighbor_hash || i == *other_i {
                    continue;
                }             

                let sqr_distance = self.particles.predicted_positions[i].distance_to_squared(self.particles.predicted_positions[*other_i]);

                if sqr_distance < sqr_radius {
                    neighbors.push(*other_i);
                }
            }
        }

        neighbors
    }

    fn viscosity_kernel(&self, distance: f32) -> f32 {
        if distance >= self.particles.smoothing_radius {
            return 0.0;
        }

        let volume = PI * self.particles.smoothing_radius.powf(8.0) / 4.0;
        let value = self.particles.smoothing_radius * self.particles.smoothing_radius - distance * distance;

        value * value * value / volume
    }

    fn calculate_viscosity_force(&self, i: usize) -> Vector2 {
        let mut viscosity_force = Vector2::zero();

        // OPTIMIZATION: only search for neighbors in the cells grid
        for other_i in self.in_radius_neighbors_search(i) {
            let distance = self.particles.predicted_positions[i].distance_to(self.particles.predicted_positions[other_i]);
            let influecne = self.viscosity_kernel(distance);
            let velocity_difference = self.particles.velocities[other_i] - self.particles.velocities[i];
            viscosity_force += velocity_difference * influecne;
        }

        // NO OPTIMIZATION: search for all neighbors
        //for other_i in 0..self.particles.len() {
        //    let distance = self.particles.predicted_positions[i].distance_to(self.particles.predicted_positions[other_i]);
        //    let influecne = self.viscosity_kernel(distance);
        //    let velocity_difference = self.particles.velocities[other_i] - self.particles.velocities[i];
        //    viscosity_force += velocity_difference * influecne;
        //}

        viscosity_force * self.viscosity_strength
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

        // OPTIMIZATION: only search for neighbors in the cells grid
        for other_i in self.in_radius_neighbors_search(i) {
            let offset = self.particles.predicted_positions[i] - self.particles.predicted_positions[other_i];
            let distance = offset.magnitude();
            let direction = if distance == 0.0 { Vector2::random() } else { offset / distance };
            let slope = self.smoothing_kernel_derivative(distance);
            let density = self.particles.densities[other_i];
            let shared_pressure = self.calculate_shared_pressure(i, other_i);

            pressure_force += direction * shared_pressure * slope * self.particles.mass / density;
        }

        // NO OPTIMIZATION: search for all neighbors
        //for other_i in 0..self.particles.len() {
        //    let offset = self.particles.predicted_positions[i] - self.particles.predicted_positions[other_i];
        //    let distance = offset.magnitude();
        //    let direction = if distance == 0.0 { Vector2::random() } else { offset / distance };
        //    let slope = self.smoothing_kernel_derivative(distance);
        //    let density = self.particles.densities[other_i];
        //    let shared_pressure = self.calculate_shared_pressure(i, other_i);
//
        //    pressure_force += direction * shared_pressure * slope * self.particles.mass / density;
        //}

        pressure_force
    }

    fn calculate_interaction_force(&self, position: Vector2, radius: f32, strength: f32, i: usize) -> Vector2 {
        let mut interaction_force = Vector2::zero();
        let dst = self.particles.positions[i].distance_to(position);

        if dst < radius {
            let direction = if dst <= f32::EPSILON { Vector2::zero() } else { (position - self.particles.positions[i]).normalize().unwrap() };
            let center_t = 1.0 - dst / radius;
            interaction_force += (direction * strength - self.particles.velocities[i]) * center_t;
        }

        interaction_force * if self.interactive_force_mode { 1.0 } else { -0.15 }
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
        if self.interactive_force {
            // Apply gravity and predicted positions
            (0..self.particles.len()).for_each(|i| {
                self.particles.velocities[i] += Vector2::down() * self.gravity * dt;
                let interaction_force = self.calculate_interaction_force(self.interactive_force_position, 100.0, 150.0, i);
                self.particles.velocities[i] += interaction_force;
                self.particles.predicted_positions[i] = self.particles.positions[i] + self.particles.velocities[i] * dt;
            });
        } else {
            // Apply gravity and predicted positions
            (0..self.particles.len()).for_each(|i| {
                self.particles.velocities[i] += Vector2::down() * self.gravity * dt;
                self.particles.predicted_positions[i] = self.particles.positions[i] + self.particles.velocities[i] * dt;
            });
        }

        // Update spatial lookup
        self.particles.update_spatial_lookup();

        if self.visual_filter == 3 {
            // Calculate densities
            (0..self.particles.len()).for_each(|i| {
                self.particles.densities[i] = self.calculate_density(i);
                self.particles.colors[i] = self.density_gradient.at((self.particles.densities[i] * 100.0 / self.particles.target_density * 2.0) as f64).to_hex_string();
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
                self.particles.colors[i] = self.velocity_gradient.at(pressure_force.magnitude() as f64).to_hex_string();
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
            let viscosity_force = self.calculate_viscosity_force(i);
            self.particles.velocities[i] += viscosity_force * dt;
            self.particles.positions[i] += self.particles.velocities[i] * dt;
            self.resolve_collision(i);
        });

        //dbg!(&self.particles);
    }
}
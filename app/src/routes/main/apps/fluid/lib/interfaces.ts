import type { Vector2 } from "$lib/components/app/Interfaces/vector2";

/*
pub struct FluidParticles {
    // ALL PARTICLES PROPERTIES
    pub mass: f32,
    pub radius: f32,
    pub target_density: f32, 
    pub pressure_multiplier: f32,
    pub smoothing_radius: f32,
    // EACH PARTICLE PROPERTIES
    pub positions: Vec<Vector2>,
    pub velocities: Vec<Vector2>,
    pub densities: Vec<f32>,
    pub colors: Vec<String>, // store in hex string format and not in colorgrad::Color to allow serialization
}
*/

export interface FluidStarterData {
    positions: Vector2[],
}

export interface FluidParticles {
    mass: number,
    radius: number,
    target_density: number,
    pressure_multiplier: number,
    smoothing_radius: number,
    positions: Vector2[],
    velocities: Vector2[],
    densities: number[],
    colors: string[],
}

export interface RendererData {
    fluid_particles: FluidParticles,
}
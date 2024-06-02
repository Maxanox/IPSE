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




export interface AABB {
    min: Vector2;
    max: Vector2;
}


export enum ShapeType {
    Circle = 0,
    Box = 1
}

export interface RigidBody {
    position: Vector2;
    linearVelocity: Vector2;
    angle: number;
    angularVelocity: number;
    force: Vector2;
    mass: number;
    invMass: number;
    density: number;
    area: number;
    restitution: number;
    isStatic: boolean;
    radius: number;
    width: number;
    height: number;
    shape: ShapeType;
    triangles: number[];
    vertices: Vector2[];
    transformedVertices: Vector2[];
    aabb: AABB;
    tfvRequired: boolean;
    aabbUpdate: boolean;
    index: number;
    inertia: number;
    invInertia: number;
}



export interface WorkSpace {
    mnBs: number;
    mxBs: number;
    mnD: number;
    mxD: number;
    minIter: number;
    maxIter: number;
    bodyList: RigidBody[];
    gravity: Vector2;
    bodyCount: number;
    contactPair: [number, number][];
}


export interface RendererData {
    bodies: WorkSpace,
}

/*
pub struct EventSettings {
    pub collision_restitution: f32,
    pub gravity: f32,
    pub target_density: f32,
    pub mass: f32,
    pub pressure_stiffness: f32,
    pub visual_filter: u8,
    pub smoothing_radius: f32
}
*/


export interface EventSettings {
    collision_restitution: number,
    gravity: number,
    target_density: number,
    mass: number,
    pressure_stiffness: number,
    visual_filter: number,
    smoothing_radius: number
}
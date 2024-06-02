import type { Vector2 } from "$lib/components/app/Interfaces/vector2";


/*
pub struct LightRigidBody {
    pub position: Vector2,
    pub rotation: f32,
    pub radius: f32,
    pub width: f32,
    pub height: f32,
    pub shape: bool,
}
 */

export interface LightRigidBody {
    position: Vector2;
    rotation: number;
    radius: number;
    width: number;
    height: number;
    shape: boolean;
}

export interface RendererData {
    bodies: LightRigidBody[];
}

import type { Vector2 } from "$lib/components/app/Interfaces/vector2";

export interface Ball {
    position: Vector2,
    velocity: Vector2,
    radius: number,
    mass: number,
    color: string,
}
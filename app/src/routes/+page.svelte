<script lang="ts">
    import { Application, ParticleContainer } from 'svelte-pixi';
    import { LightSwitch } from '@skeletonlabs/skeleton';
    import { listen } from '@tauri-apps/api/event';
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMount } from 'svelte';
    import * as PIXI from 'pixi.js';

    let width = window.innerWidth;
    let height = window.innerHeight;

    let viewWidth = width * 0.9;
    let viewHeight = height * 0.75;

    let step = 0;
    let fps = 0;

    let start = performance.now();
    let duration = 0;

    let particleContainer: any;
    let particle_sprites: PIXI.Sprite[] = [];
    let particle_coords: Vector2[] = [];

    class FPSCounter {
        smoothingFactor: number;
        frameCount: number;
        lastUpdateTime: number;
        fps: number;

        constructor(smoothingFactor = 0.9) {
            this.smoothingFactor = smoothingFactor;
            this.frameCount = 0;
            this.lastUpdateTime = performance.now();
            this.fps = 0;
        }

        update() {
            const now = performance.now();
            const deltaTime = (now - this.lastUpdateTime) / 1000;
            this.lastUpdateTime = now;

            this.frameCount++;
            const instantFPS = 1 / deltaTime;

            // Exponential moving average
            this.fps = this.smoothingFactor * this.fps + (1 - this.smoothingFactor) * instantFPS;

            return this.fps;
        }
    }

    const fpsCounter = new FPSCounter();

    const unlisten_nextStep = listen('next-step', () => {
        step++;
        fps = fpsCounter.update();
    });

    interface Vector2 {
        x: number;
        y: number;
    }

    interface RenderPayload {
        positions: Vector2[];
    }

    const unlistnen_drawParticles = listen('draw-particles', (event) => {
        const payload = event.payload as RenderPayload;
        particle_sprites.forEach((particle, index) => {
            particle.x = payload.positions[index].x;
            particle.y = payload.positions[index].y;
        });
    });

    function getRandomInt(min: number, max: number): number {
        min = Math.ceil(min);
        max = Math.floor(max);
        return Math.floor(Math.random() * (max - min + 1)) + min;
    }

    onMount(() => {
        for (let i = 0; i < 5000; i++) {
            const particle = new PIXI.Sprite(PIXI.Texture.from('src/static/assets/particle.png'));
            particle.tint = 0x0077ff; // Set particle color to ideal blue resembling water
            particle.scale.set(0.1);
            particle.anchor.set(0.5, 0.5);
            particle.x = getRandomInt(0, viewWidth);
            particle.y = getRandomInt(0, viewHeight);
            particle_sprites.push(particle);
            particle_coords.push({ x: particle.x, y: particle.y });
        }
        
        particleContainer.addChild(...particle_sprites);

        invoke('start_simulation', { width: viewWidth, height: viewHeight, particles: particle_coords });

        return async () => {
            await unlisten_nextStep;
            await unlistnen_drawParticles;

            particle_sprites.forEach(particle => particle.destroy());
        };
    });
</script>

<main class="container">
    <div class="flex flex-col items-center justify-center p-5">
        <div class="card flex flex-row items-center justify-left w-full h-11 p-5 gap-5">
            <span>Step : {step}</span>
            <span class="divider-vertical h-6 m-0"/>
            <span>FPS : {fps.toFixed(0)}/80</span>
            <span class="divider-vertical h-6 m-0"/>
            <span>Duration : {duration} sec</span>
            <LightSwitch class="ml-auto"/>
        </div>
        <div class="card m-5">
            <Application width={viewWidth} height={viewHeight} backgroundAlpha={0} antialias>
                <ParticleContainer
                    bind:instance={particleContainer}
                    autoResize
                    properties={{
                        scale: true,
                        position: true,
                        rotation: true
                    }}
                />
            </Application>
        </div>
    </div>
</main>


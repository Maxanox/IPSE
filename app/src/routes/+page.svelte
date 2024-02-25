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
    let particles: PIXI.Sprite[] = [];

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
        particles.forEach((particle, index) => {
            particle.x = payload.positions[index].x;
            particle.y = payload.positions[index].y;
        });
    });

    onMount(() => {
        const particle = new PIXI.Sprite(PIXI.Texture.from('src/static/assets/particle.png'));
        particle.tint = 0x0077ff; // Set particle color to ideal blue resembling water
        particle.scale.set(0.2);
        particle.x = 100.0;
        particle.y = 100.0;
        particles.push(particle);
        invoke('add_particle');

        particleContainer.addChild(...particles);

        invoke('start_simulation');

        return async () => {
            await unlisten_nextStep;
            await unlistnen_drawParticles;

            particles.forEach(particle => particle.destroy());
        };
    });
</script>

<main class="container">
    <div class="flex flex-col items-center justify-center p-5">
        <div class="card flex flex-row items-center justify-left w-full h-11 p-5 gap-5">
            <span>Step : {step}</span>
            <span class="divider-vertical h-6 m-0"/>
            <span>FPS : {fps.toFixed(0)}/60</span>
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


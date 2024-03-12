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
    let duration = 0;

    let particleCount = 0;
    let particleToSpawn = 1;

    let particleContainer: any;
    let particle_sprites: PIXI.Sprite[] = [];

    let spawn_button: HTMLButtonElement;

    type Timeout = ReturnType<typeof setInterval>;
    let duration_id: Timeout;

    enum SimulationTemplateEnum {
        BouncingBalls
    }

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

    interface Vector2 {
        x: number;
        y: number;
    }

    interface RenderPayload {
        positions: [Vector2, string][];
    }

    const unlistnen_drawParticles = listen('draw', (event) => {
        const payload = event.payload as RenderPayload;
        (particleContainer as PIXI.ParticleContainer).children.forEach((particle, index) => {
            particle.x = payload.positions[index][0].x;
            particle.y = payload.positions[index][0].y;
            particle.tint = new PIXI.Color(payload.positions[index][1]);
        });
        step++;
        fps = fpsCounter.update();
    });

    function getRandomInt(min: number, max: number): number {
        min = Math.ceil(min);
        max = Math.floor(max);
        return Math.floor(Math.random() * (max - min + 1)) + min;
    }

    let buttonPreviousStep: HTMLButtonElement;
    let buttonNextStep: HTMLButtonElement;

    async function pauseSimulation() {
        await invoke('pause_simulation');
        buttonPreviousStep.disabled = !buttonPreviousStep.disabled;
        buttonNextStep.disabled = !buttonNextStep.disabled;
    }

    let launched = false;

    async function spawnParticles() {
        launched = true;
        spawn_button.disabled = true;

        invoke('run_simulation');

        let particle_coords: Vector2[] = [];

        for (let i = 0; i < particleToSpawn; i++) {
            const particle = new PIXI.Sprite(PIXI.Texture.from('src/static/assets/particle.png'));
            //particle.tint = 0x0077ff; // Set particle color to ideal blue resembling water
            particle.scale.set(0.1);
            particle.anchor.set(0.5, 0.5);
            particle.x = getRandomInt(0, viewWidth);
            particle.y = getRandomInt(0, viewHeight);
            particle_sprites.push(particle);
            particle_coords.push({ x: particle.x, y: particle.y });
        }
        
        particleCount += particleToSpawn;

        particleContainer.addChild(...particle_sprites);

        await invoke('add_particles', { particles: particle_coords });

        duration_id = setInterval(() => {
            duration += 0.01;
        }, 10);
    }

    async function nextStep() {
        await invoke('next_step');
    }

    async function previousStep() {
        await invoke('previous_step');
    }

    onMount(() => {
        invoke('select_simulation_template', { width: viewWidth, height: viewHeight });

        return async () => {
            await unlistnen_drawParticles;

            clearInterval(duration_id);

            particle_sprites.forEach(particle => particle.destroy());
        };
    });
</script>

<main class="container m-auto">
    <div class="flex flex-col items-center justify-center p-5">
        <div class="card flex flex-row items-center justify-left w-full h-11 p-5 gap-5">
            <span>Step : {step}</span>
            <span class="divider-vertical h-6 m-0"/>
            <span>FPS : {fps.toFixed(1)}/60</span>
            <span class="divider-vertical h-6 m-0"/>
            <span>Duration : { (duration).toFixed(2) } sec</span>
            <span class="divider-vertical h-6 m-0"/>
            <span>Particles : { particleCount }</span>
            <LightSwitch class="ml-auto"/>
        </div>
        <div class="card m-5">
            <Application width={viewWidth} height={viewHeight} backgroundAlpha={0} antialias>
                <ParticleContainer
                    bind:instance={particleContainer}
                    autoResize
                    properties={{
                        position: true,
                        tint: true
                    }}
                />
            </Application>
        </div>

        {#if !launched}
            <label>
                Particle Count: {particleToSpawn}
                <input type="range" bind:value={particleToSpawn} min="0" max="5000" />
            </label>
            <button bind:this={spawn_button} type="button" class="btn variant-filled" on:click={spawnParticles}>Spawn</button>
        {:else}
            <div class="card flex flex-row items-center justify-center p-2 gap-5">
                <button bind:this={buttonPreviousStep} type="button" on:click={nextStep} class="btn variant-filled">{"<"}</button>
                <button type="button" class="btn variant-filled" on:click={pauseSimulation}>=</button>
                <button bind:this={buttonNextStep} type="button" on:click={previousStep} class="btn variant-filled">{">"}</button>
            </div>
            <label>
                <input type="range" bind:value={particleToSpawn} min="0" max="5000" />
            </label>
        {/if}
        </div>
</main>


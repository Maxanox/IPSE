<script lang="ts">
    import { Application, ParticleContainer } from 'svelte-pixi';
    import { LightSwitch, AppBar } from '@skeletonlabs/skeleton';
    import { listen } from '@tauri-apps/api/event';
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMount } from 'svelte';
    import * as PIXI from 'pixi.js';

    import HBarQuickData from '$lib/components/app/UI/boxes/HBarQuickData.svelte';

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

    interface Ball {
        position: Vector2,
        velocity: Vector2,
        radius: number,
        mass: number,
        color: string,
    }

    let x: number, y: number;

    const unlistnen_drawParticles = listen('render', (event) => {
        const payload = event.payload as Ball[];
        (particleContainer as PIXI.ParticleContainer).children.forEach((particle, index) => {
            particle.x = payload[index].position.x;
            x = particle.x;
            particle.y = y = payload[index].position.y;
            y = particle.y;
            particle.tint = new PIXI.Color(payload[index].color);
            particle.scale.set(payload[index].radius/64);
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
    let err: any;

    async function spawnParticles() {
        launched = true;
        spawn_button.disabled = true;

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

        await invoke('run_simulation').catch((error) => err = error);

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

    import App from '$lib/components/app/AppBase/App.svelte';
</script>

<App regionPage="relative" slotPageHeader="sticky top-0 z-10">
    <svelte:fragment slot="header">
        <AppBar>
            <svelte:fragment slot="lead">(icon)</svelte:fragment>
            (title)
            <svelte:fragment slot="trail">(actions)</svelte:fragment>
        </AppBar>
    </svelte:fragment>
    <svelte:fragment slot="pageHeader">
        <HBarQuickData
            class_="m-5"
            data={[
                { name: 'Step', value: step }, 
                { name: 'FPS', value: fps.toFixed(2) }, 
                { name: 'Duration', value: duration.toFixed(2) }
            ]} 
            light_switch={true}>
        </HBarQuickData>
    </svelte:fragment>

    <div class="flex flex-col items-center justify-center p-5">
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
            {"(x, y) : (" + x.toFixed(2) + ", " + y.toFixed(2) + ")"}

            <span class="text-red-500">{err}</span>
        {/if}
    </div>
</App>
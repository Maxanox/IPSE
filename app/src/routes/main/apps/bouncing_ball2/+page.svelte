<script lang="ts">
    import { listen, type UnlistenFn } from '@tauri-apps/api/event';
    import { invoke } from '@tauri-apps/api/tauri';
    import { onDestroy, onMount } from 'svelte';
    
    import type { Vector2 } from '$lib/components/app/Interfaces/vector2.ts';
    import App from '$lib/components/app/App/App.svelte';
    import Renderer from '$lib/components/app/Renderer/Renderer.svelte';

    import * as PIXI from 'pixi.js';

    import { ParticleContainer } from 'svelte-pixi';
    import HBarQuickData from '$lib/components/app/UI/boxes/HBarQuickData.svelte';
    
    import type { Ball, RendererData } from './lib/interfaces';
    import ParticleSrc from "./static/particle.png";

    let duration_callback: NodeJS.Timeout;

    let renderer_width: number;
    let renderer_height: number;
    let particle_container: PIXI.ParticleContainer;

    let step = 0;
    let fps = 120;
    let duration = 0;

    let launched = false;

    let particle_number = 1;
    let speed_coef = 1;
    let err = "";

    function getRandomInt(max: number) {
        return Math.floor(Math.random() * max);
    }

    interface BouncingBallStarterData {
        positions: Vector2[];
    }

    async function spawnParticles() {
        launched = true;

        let starter_data: BouncingBallStarterData = { positions: [] };

        for (let i = 0; i < particle_number; i++) {
            let particle = new PIXI.Sprite(PIXI.Texture.from(ParticleSrc));
            // particle.tint = 0x0077ff; // Set particle color to ideal blue resembling water
            particle.anchor.set(0.5, 0.5);
            particle.x = getRandomInt(renderer_width);
            particle.y = getRandomInt(renderer_height);
            starter_data.positions.push({ x: particle.x, y: particle.y });
            particle_container.addChild(particle);
        }

        let renderer_size: Vector2 = { x: renderer_width, y: renderer_height };

        await invoke('initialize_simulation', { rendererSize: renderer_size, serializedData: JSON.stringify(starter_data)}).catch((error) => err = error);
        await invoke('run_simulation').catch((error) => err = error);

        duration_callback = setInterval(() => {
            duration += 0.01;
        }, 10);
    }

    let unlistnen_drawParticles: UnlistenFn;

    onMount(async () => {
        await invoke('select_simulation_template', { width: renderer_width, height: renderer_height, id: 1 }).catch((error) => err = error);

        unlistnen_drawParticles = await listen('render', async (event) => {
            let payload = event.payload as RendererData;

            if (payload.balls.length !== particle_container.children.length) {
                err = "Particle count mismatch : " + payload.balls.length + " != " + particle_container.children.length;
            }
            else if (err === "Particle count mismatch") {
                err = "";
            }

            particle_container.children.forEach((particle, index) => {
                particle.x = payload.balls[index].position.x;
                particle.y = payload.balls[index].position.y;
                particle.tint = parseInt(payload.balls[index].color.replace("#", "0x"));
                particle.scale.set(payload.balls[index].radius/64);
            });

            step++;
        });
    });

    onDestroy(async () => {
        clearInterval(duration_callback);

        unlistnen_drawParticles();

        await invoke('quit_simulation').catch((error) => err = error);
    });
</script>

<App slotPageHeader="flex" regionPage="p-5 gap-5">
    <svelte:fragment slot="pageHeader">
        <HBarQuickData
            data={[
                { name: 'Step', value: step }, 
                { name: 'FPS', value: fps.toFixed(2) }, 
                { name: 'Duration', value: duration.toFixed(2) },
                { name: 'Particles', value: particle_container?.children.length }
            ]} 
            light_switch={true}>
        </HBarQuickData>
    </svelte:fragment>

    <!-- default slot -->
    <div class="flex flex-col items-center justify-center gap-5">
        <Renderer bind:width={renderer_width} bind:height={renderer_height} controls={false}>
            <ParticleContainer
                bind:instance={particle_container}
                autoResize
                properties={{
                    position: true,
                    tint: true,
                    scale: true
                }}
            />
        </Renderer>
        
        {#if !launched}
            <label>
                Particle Count: {particle_number}
                <input type="range" bind:value={particle_number} min="1" max="500" />
            </label>
            <button type="button" class="btn variant-filled" on:click={spawnParticles}>Spawn</button>
        {:else}
            <label>
                <input type="range" bind:value={speed_coef} min="0" max="10" />
            </label>

            <span class="text-red-500">{err}</span>
        {/if}
    </div>
    <!-- /default slot -->
</App>
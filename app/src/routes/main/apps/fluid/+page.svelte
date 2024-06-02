<script lang="ts">
    import { listen, type UnlistenFn } from '@tauri-apps/api/event';
    import { invoke } from '@tauri-apps/api/tauri';
    import { onDestroy, onMount } from 'svelte';
    import { RadioGroup, RadioItem } from '@skeletonlabs/skeleton'
    
    import type { Vector2 } from '$lib/components/app/Interfaces/vector2.ts';
    import App from '$lib/components/app/App/App.svelte';
    import Renderer from '$lib/components/app/Renderer/Renderer.svelte';

    import * as PIXI from 'pixi.js';

    import { Container, Graphics } from 'svelte-pixi';
    import HBarQuickData from '$lib/components/app/UI/boxes/HBarQuickData.svelte';
    
    import type { FluidStarterData, RendererData, EventSettings } from './lib/interfaces';
    import ParticleSrc from "./static/particle.png";
    import { fade } from 'svelte/transition';

    let duration_callback: NodeJS.Timeout;

    let renderer_div_owner: HTMLDivElement;
    let renderer_width: number;
    let renderer_height: number;
    let particle_container: PIXI.Container;

    let step = 0;
    let fps = 120;
    let duration = 0;

    let launched = false;
    let running = true;

    let particle_number = 820;
    let err = "";

    let drag = false;
    let mouse_position: Vector2 = { x: 0, y: 0 };

    let event_settings: EventSettings = { 
        collision_restitution: 0.95,
        gravity: 60,
        target_density: 2,
        mass: 1,
        pressure_stiffness: 100,
        visual_filter: 0,
        smoothing_radius: 30,
        viscosity_strength: 1,
        interactive_force_mode: true,
    };

    $ : {
        invoke('send_event_to_simulation', { event: "set_settings", data: JSON.stringify(event_settings) }).catch((error) => err = error);
    }

    $ : {
        invoke('send_event_to_simulation', { event: "interractive_force_toggle", data: JSON.stringify(drag) }).catch((error) => err = error);
    }

    $ : {
        if (renderer_div_owner) {
            renderer_width = renderer_div_owner.clientWidth;
            renderer_height = renderer_div_owner.clientHeight;
        }
    }

    function getRandomInt(max: number) {
        return Math.floor(Math.random() * max);
    }

    async function selectSimulation() {
        await invoke('select_simulation_template', { width: 0, height: 0, id: 1 }).catch((error) => err = error);

        unlistnen_drawParticles = await listen('render', async (event) => {
            let payload = event.payload as RendererData;
            let fluid_particles = payload.fluid_particles;
            
            if (!payload) {
                err = "No payload received";
                return;
            }
            else if (fluid_particles.positions.length !== particle_container.children.length) {
                err = "Particle count mismatch : " + fluid_particles.positions.length + " != " + particle_container.children.length;
            }
            else if (err.startsWith("Particle count mismatch")) {
                err = "";
            }

            particle_container.children.forEach((particle, index) => {
                let sprite = particle as PIXI.Sprite;
                sprite.x = fluid_particles.positions[index].x;
                sprite.y = fluid_particles.positions[index].y;
                sprite.tint = parseInt(fluid_particles.colors[index].replace("#", "0x"));
                sprite.scale.set(fluid_particles.radius/64);
            });

            step++;
        });
    }

    async function initSimulation() {
        step = 0;
        fps = 120;
        duration = 0;

        launched = false;
        running = true;

        err = "";

        drag = false;
        mouse_position = { x: 0, y: 0 };

        let starter_data: FluidStarterData = { positions: [] };

        const squareSize = renderer_height / 1.5;
        const squareX = (renderer_width - squareSize) / 2;
        const squareY = (renderer_height - squareSize) / 2;

        let particleIndex = 0;
        for (let y = squareY; y < squareY + squareSize; y += squareSize / Math.sqrt(particle_number)) {
            for (let x = squareX; x < squareX + squareSize; x += squareSize / Math.sqrt(particle_number)) {
                let particle = new PIXI.Sprite(PIXI.Texture.from(ParticleSrc));
                particle.anchor.set(0.5, 0.5);
                particle.x = x;
                particle.y = y;
                starter_data.positions.push({ x: particle.x, y: particle.y });
                particle_container.addChild(particle);
                particleIndex++;
                if (particleIndex >= particle_number) {
                    break;
                }
            }
        }

        let renderer_size: Vector2 = { x: renderer_width, y: renderer_height };

        await invoke('initialize_simulation', { rendererSize: renderer_size, serializedData: JSON.stringify(starter_data)}).catch((error) => err = error);

        await update_settings();
    }

    async function runSimulation() {
        if (launched) {
            await invoke('run_simulation').catch((error) => err = error);
        } else {
            await selectSimulation();
            await initSimulation();
            await invoke('run_simulation').catch((error) => err = error);
            launched = true;
        }

        particle_container.visible = true;
        running = true;

        duration_callback = setInterval(() => {
            duration += 0.01;
        }, 10);
    }

    async function stopSimulation() {
        running = false;

        clearInterval(duration_callback);

        await invoke('stop_simulation').catch((error) => err = error);
    }

    async function quitSimulation() {
        particle_container.visible = false;
        running = false;
        launched = false;

        clearInterval(duration_callback);

        err = "";
        duration = 0;
        step = 0;
        particle_container.removeChildren();

        unlistnen_drawParticles();

        await invoke('quit_simulation').catch((error) => err = error);
    }

    async function resetSimulation() {
        await quitSimulation();
        await runSimulation();
    }

    async function update_settings() {
        await invoke('send_event_to_simulation', { event: 'set_settings', data: JSON.stringify(event_settings) }).catch((error) => err = error);
    }

    let unlistnen_drawParticles: UnlistenFn;

    onMount(async () => {
        selectSimulation();
    });

    onDestroy(async () => {
        quitSimulation();
    });

    async function interactive_force_position_update() {
        await invoke('send_event_to_simulation', { event: 'interactive_force_position', data: JSON.stringify({ x: mouse_position.x, y: mouse_position.y }) }).catch((error) => err = error);
    }
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
    <div class="flex flex-row h-full gap-5">
        <div class="card p-4 flex flex-col gap-5">
            <label>
                <input type="number" class="badge variant-filled mr-1" 
                    bind:value={event_settings.gravity} min={0} max={100} step={0.5}
                />
                <span>Gravity</span>
                <input type="range" bind:value={event_settings.gravity} min={0} max={100} step={0.01}/>
            </label>

            <label>
                <input type="number" class="badge variant-filled mr-1" 
                    bind:value={event_settings.collision_restitution} min={0} max={1} step={0.5}
                />
                <span>Collision restitution</span>
                <input type="range" bind:value={event_settings.collision_restitution} min={0} max={1} step={0.01}/>
            </label>

            <label>
                <input type="number" class="badge variant-filled mr-1" 
                    bind:value={event_settings.viscosity_strength} min={0} max={1} step={0.5}
                />
                <span>Viscosity Strenght</span>
                <input type="range" bind:value={event_settings.viscosity_strength} min={0} max={1} step={0.01}/>
            </label>

            <label>
                <input type="number" class="badge variant-filled mr-1" 
                    bind:value={event_settings.pressure_stiffness} min={0} max={100} step={0.5}
                />
                <span>Pressure Multiplier</span>
                <input type="range" bind:value={event_settings.pressure_stiffness} min={0} max={100} step={0.1}/>
            </label>

            <label>
                <input type="number" class="badge variant-filled mr-1" 
                    bind:value={event_settings.target_density} min={0} max={1000} step={0.5}
                />
                <span>Target Density</span>
                <input type="range" bind:value={event_settings.target_density} min={0} max={1000} step={0.1}/>
            </label>

            <label>
                <input type="number" class="badge variant-filled mr-1" 
                    bind:value={event_settings.smoothing_radius} min={1} max={100} step={0.5}
                />
                <span>Smoothing Radius</span>
                <input type="range" bind:value={event_settings.smoothing_radius} min={1} max={100} step={0.1}/>
            </label>

            <label class="label">
                <span>Visual Filter</span>
                <select class="select" bind:value={event_settings.visual_filter}>
                    <option value={0}>Default</option>
                    <option value={1}>Velocity</option>
                    <option value={2}>Pressure</option>
                    <option value={3}>Density</option>
                </select>
            </label>

            <RadioGroup>
                <RadioItem bind:group={event_settings.interactive_force_mode} name="Pull" value={true}><i class="fa-solid fa-minimize"></i></RadioItem>
                <RadioItem bind:group={event_settings.interactive_force_mode} name="Push" value={false}><i class="fa-solid fa-maximize"></i></RadioItem>
            </RadioGroup>

            {#if !launched}
                <div class="flex flex-col justify-center items-center my-auto gap-2">
                    <label class="flex flex-col w-2/3">
                        <span>Particle Count: {particle_number}</span>
                        <input type="range" bind:value={particle_number} min="1" max="3000" />
                    </label>
                    <button type="button" class="btn variant-filled" on:click={runSimulation}>Spawn</button>
                </div>
            {:else}
                <span class="text-red-500 m-auto">{err}</span>
            {/if}

            {#if launched}
                <div class="flex flex-row gap-2 justify-center w-full">
                    <button type="button" class="w-full bg-orange-500 hover:bg-orange-600 text-white p-2 rounded-lg" on:click={resetSimulation}>
                        <i class="fa-solid fa-rotate-left"></i>
                    </button>
                    {#if running}
                        <button type="button" class="w-full bg-blue-500 hover:bg-blue-600 text-white p-2 rounded-lg" on:click={stopSimulation}>
                            <i class="fa-solid fa-pause"></i>
                        </button>
                    {:else}
                        <button type="button" class="w-full bg-green-500 hover:bg-green-600 text-white p-2 rounded-lg" on:click={runSimulation}>
                            <i class="fa-solid fa-play"></i>
                        </button>
                    {/if}
                    <button type="button" class="w-full bg-red-500 hover:bg-red-600 text-white p-2 rounded-lg" on:click={quitSimulation}>
                        <i class="fa-solid fa-stop"></i>
                    </button>                
                </div>
            {/if}
        </div>
        
        <div bind:this={renderer_div_owner} class="flex items-center justify-center w-full h-full relative"
            on:pointerdown={(event) => {
                mouse_position = { x: Math.round(event.offsetX), y: Math.round(event.offsetY) }; 
                drag = true
                interactive_force_position_update();
            }}
            on:pointerup={() => drag = false}
            on:pointerleave={() => drag = false}
            on:pointermove={(event) => {
                if (drag) {
                    mouse_position = { x: Math.round(event.offsetX), y: Math.round(event.offsetY) };
                    interactive_force_position_update();
                }
            }}
        >
            {#if !renderer_width || !renderer_height}
                <span>Loading...</span>
            {:else}
                <Renderer width={renderer_width} height={renderer_height} controls={false}>
                    <Container bind:instance={particle_container}/>
                    {#if drag}
                        <Graphics
                            x={mouse_position.x}
                            y={mouse_position.y}
                            draw={(g) => {
                                g.lineStyle(1, 0x00AA00);
                                g.drawCircle(0, 0, 100);
                            }}
                        />
                        <div transition:fade class="absolute z-10 top-2 left-2 text-white bg-black bg-opacity-20 p-2 rounded-lg">
                            {'(' + mouse_position.x + ',' + mouse_position.y + ')'}
                        </div>
                    {/if}
                </Renderer>
            {/if}
        </div>
    </div>
    <!-- /default slot -->
</App>
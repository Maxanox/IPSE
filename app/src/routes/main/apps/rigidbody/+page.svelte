<script lang="ts">
    import { listen, type UnlistenFn } from '@tauri-apps/api/event';
    import { invoke } from '@tauri-apps/api/tauri';
    import { onDestroy, onMount } from 'svelte';
    
    import type { Vector2 } from '$lib/components/app/Interfaces/vector2.ts';
    import App from '$lib/components/app/App/App.svelte';
    import Renderer from '$lib/components/app/Renderer/Renderer.svelte';

    import * as PIXI from 'pixi.js';

    import { Container } from 'svelte-pixi';
    
    import type { RendererData } from './lib/interfaces';

    let renderer_width: number = 1000;
    let renderer_height: number = 600;   

    let launched = false;

    let unlistnen_render: UnlistenFn;

    let container: PIXI.Container;

    async function startSimulation() {
        launched = true;

        let renderer_size: Vector2 = { x: renderer_width, y: renderer_height };

        await invoke('initialize_simulation', { rendererSize: renderer_size, serializedData: null});

        await invoke('run_simulation');
    }

    onMount(async () => {
        await invoke('select_simulation_template', { width: renderer_width, height: renderer_height, id: 2 });
        unlistnen_render = await listen('render', async (event) => {
            let payload = event.payload as RendererData;

            container.removeChildren();

            for (let i = 0; i < payload.bodies.length; i++) {
                let body = payload.bodies[i];
                
                let graphics = new PIXI.Graphics()

                // Si c'est une Box
                if (body.shape) {
                    graphics
                        .beginFill(parseInt("0xFFFFFFFF"))
                        .drawRect(body.position.x, body.position.y, body.width, body.height)
                        .endFill();
                } else { // Sinon c'est un cercle
                    graphics
                        .beginFill(parseInt("0xFFFFFFFF"))
                        .drawCircle(body.position.x, body.position.y, body.radius)
                        .endFill();
                }
                
                container.addChild(graphics);

                graphics.rotation = body.rotation;
            }
        });
    });

    onDestroy(async () => {
        unlistnen_render();
        await invoke('quit_simulation');
    });
</script>

<App slotPageHeader="flex" regionPage="p-5 gap-5">
    <!-- default slot -->
    <div class="flex flex-col items-center justify-center gap-5 m-auto">
        <Renderer bind:width={renderer_width} bind:height={renderer_height} controls={false}>
            <Container bind:instance={container} />
        </Renderer>
        
        {#if !launched}
            <button type="button" class="btn variant-filled" on:click={startSimulation}>Start</button>
        {/if}
    </div>
    <!-- /default slot -->
</App> 
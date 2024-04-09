<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { Application } from 'svelte-pixi';

    export let width = 800;
    export let height = 600;

    export let controls = true;

    export let class_ = "";

    async function nextStep() {
        await invoke('next_step');
    }

    async function previousStep() {
        await invoke('previous_step');
    }

</script>

<div class="card {class_}">
    <Application width={width} height={height} backgroundAlpha={0} antialias>
        <slot />
    </Application>
</div>

{#if controls}
    <div class="card flex flex-row items-center justify-center p-2 gap-5">
        <button type="button" class="btn variant-filled">{'<'}</button>
        <button type="button" class="btn variant-filled" >{'='}</button>
        <button type="button" class="btn variant-filled">{'>'}</button>
    </div>
{/if}
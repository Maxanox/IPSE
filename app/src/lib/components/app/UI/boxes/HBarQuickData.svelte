<script lang="ts">
    import { LightSwitch } from '@skeletonlabs/skeleton';

    export let class_ = '';

    //export let height = 11;

    //$: divider_height = Math.floor(height / 2);

    export let light_switch = false;

    interface QuickData {
        name: string;
        value: string | number | boolean | null | undefined;
    }

    export let data: QuickData[] = [];
</script>

<div class="card flex flex-row items-center justify-left w-full h-11 p-5 gap-5 {class_}">
    {#each data as { name, value }, i}
        <span>{name} : {value}</span>
        {#if i !== data.length - 1}
            <span class="divider-vertical m-0 h-6"/>
        {/if}
    {/each}

    {#if light_switch}
        <LightSwitch class="ml-auto"/>
    {/if}   
</div>
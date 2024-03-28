<script lang="ts">
    import { AppBar } from '@skeletonlabs/skeleton';
</script>

<AppBar>
    <svelte:fragment slot="lead">(icon)</svelte:fragment>
    (title)
    <svelte:fragment slot="trail">(actions)</svelte:fragment>
</AppBar>

<slot />
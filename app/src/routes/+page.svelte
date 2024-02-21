<script lang="ts">
    import Renderer from '$lib/Renderer.svelte';
    import { onDestroy, onMount } from 'svelte';

    let renderer: Renderer;

    let rendererWidth: number = window.innerWidth*3/4;
    let rendererHeight: number = window.innerHeight*3/4;
  
    onMount(() => {
      window.addEventListener('resize', updateRendererDimensions);
      test();
    });
  
    onDestroy(() => {
      window.removeEventListener('resize', updateRendererDimensions);
    });

    function updateRendererDimensions() {
      rendererWidth = window.innerWidth*3/4;
      rendererHeight = window.innerHeight*3/4;
    }

    function test() {
      renderer.clear();
      renderer.drawCircle(rendererWidth/2, rendererHeight/6, 40, "blue");
    }
</script>
  
<main class="container">
    <Renderer 
        bind:this={renderer} 
        width={rendererWidth} 
        height={rendererHeight}
        class_="absolute inset-1/2 -translate-x-1/2 -translate-y-1/2"
    />
</main>
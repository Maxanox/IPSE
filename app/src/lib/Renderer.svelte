<script lang="ts">  
    import { onMount } from 'svelte';

    export let width: number;
    export let height: number;
    export let class_: string;

    let canvas: HTMLCanvasElement;
    let ctx: CanvasRenderingContext2D;

    onMount(() => {
        ctx = canvas.getContext('2d')!;
        if (ctx === null) {
            throw new Error('2d context not supported');
        }
    });

    export function clear() {
        ctx.clearRect(0, 0, width, height);
    }

    export function drawRect(x: number, y: number, w: number, h: number, color: string = "white") {
        ctx.fillStyle = color;
        ctx.fillRect(x, y, w, h);
    }

    export function drawCircle(x: number, y: number, r: number, color: string = "white") {
        ctx.fillStyle = color;
        ctx.beginPath();
        ctx.arc(x, y, r, 0, Math.PI * 2);
        ctx.fill();
    }
</script>

<canvas bind:this={canvas} id="canvas" class="card {class_}" width="{width}" height="{height}"></canvas>

<style></style>
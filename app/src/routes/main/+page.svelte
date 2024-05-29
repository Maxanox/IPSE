<script>
	import { page, navigating } from '$app/stores';
</script>

<nav class="navigation">
    <a href="/main" aria-current={$page.url.pathname === '/main'}>
        main
    </a>

    <a href="/main/apps/bouncing_ball" aria-current={$page.url.pathname === '/main/apps/bouncing_ball'}>
        bouncing_ball
    </a>

    <a href="/main/apps/fluid" aria-current={$page.url.pathname === '/main/apps/fluid'}>
        fluid
    </a>
</nav>

{#if $navigating}
    <h1>navigating to {$navigating.to?.url.pathname}...</h1>
{/if}

<style>
    .navigation {
        display: flex;
        gap: 1rem;
        background-color: #f2f2f2;
        padding: 1rem;
        border-radius: 4px;
    }

    .navigation a {
        color: #333;
        text-decoration: none;
        padding: 0.5rem;
        border-radius: 4px;
        transition: background-color 0.3s ease;
    }

    .navigation a:hover {
        background-color: #ddd;
    }

    .navigation a[aria-current="true"] {
        background-color: #007bff;
        color: #fff;
    }
</style>
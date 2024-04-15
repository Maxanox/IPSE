<script lang="ts">
  import { SvelteComponent } from "svelte";
  import { AppBar } from '@skeletonlabs/skeleton';

  export let scrollbarGutter = "auto";
  export let regionPage = "";
  export let slotSidebarLeft = "w-auto";
  export let slotSidebarRight = "w-auto";
  export let slotPageHeader = "";
  export let slotPageContent = "";
  export let slotPageFooter = "";
  const cBaseAppShell = "w-full h-full flex flex-col overflow-hidden";
  const cContentArea = "w-full h-full flex overflow-hidden";
  const cPage = "flex-1 overflow-x-hidden flex flex-col";
  const cSidebarLeft = "flex-none overflow-x-hidden overflow-y-auto";
  const cSidebarRight = "flex-none overflow-x-hidden overflow-y-auto";
  $:
    classesBase = `${cBaseAppShell} ${$$props.class ?? ""}`;
  $:
    classesSidebarLeft = `${cSidebarLeft} ${slotSidebarLeft}`;
  $:
    classesSidebarRight = `${cSidebarRight} ${slotSidebarRight}`;
  $:
    classesPageHeader = `${slotPageHeader}`;
  $:
    classesPageContent = `${slotPageContent}`;
  $:
    classesPageFooter = `${slotPageFooter}`;
</script>

<div id="App" class={classesBase}>
  <header id="app-header" class="flex-none z-10">
    <AppBar>
      <svelte:fragment slot="lead">
          <a href="/main" class="btn-icon variant-filled">{"<-"}</a>
      </svelte:fragment>
      (title)
      <svelte:fragment slot="trail">(actions)</svelte:fragment>
    </AppBar>
  </header>

  <!-- Content Area -->
  <div class="flex-auto {cContentArea}">
      <!-- Slot: Sidebar (left) -->
      {#if $$slots.sidebarLeft}
          <aside id="sidebar-left" class={classesSidebarLeft}><slot name="sidebarLeft" /></aside>
      {/if}

      <!-- Page -->
      <div id="page" class="{regionPage} {cPage}" style:scrollbar-gutter={scrollbarGutter} on:scroll>
          <!-- Slot: Page Header -->
          {#if $$slots.pageHeader}
              <header id="page-header" class="flex-none {classesPageHeader}"><slot name="pageHeader">(slot:header)</slot></header>
          {/if}

          <!-- Slot: Page Content (default) -->
          <main id="page-content" class="flex-auto {classesPageContent}"><slot /></main>

          <!-- Slot: Page Footer -->
          {#if $$slots.pageFooter}
              <footer id="page-footer" class="flex-none {classesPageFooter}"><slot name="pageFooter">(slot:footer)</slot></footer>
          {/if}
      </div>

      <!-- Slot: Sidebar (right) -->
      {#if $$slots.sidebarRight}
          <aside id="sidebar-right" class={classesSidebarRight}><slot name="sidebarRight" /></aside>
      {/if}
  </div>

  <!-- <footer id="shell-footer" class="flex-none"></footer> -->
</div>
    
    
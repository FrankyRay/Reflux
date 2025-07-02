<script lang="ts">
  import type { Component } from "svelte";

  import { pageTabs } from "$lib/store/status.svelte";
  import TitleBar from "$lib/pages/download/TitleBar.svelte";
  import StatusBar from "$lib/pages/download/StatusBar.svelte";
  import Output from "$lib/components/options/Output.svelte";
  import Video from "$lib/components/options/Video.svelte";
  import Audio from "$lib/components/options/Audio.svelte";
</script>

<TitleBar />
<main>
  <div id="main-container" class="container">
    {#if pageTabs.downloadTab === "output"}
      {@render section("Output", Output)}
      {@render section("Video", Video)}
      {@render section("Audio", Audio)}
    {/if}
  </div>
</main>
<StatusBar />

{#snippet section(name: string, Comp: Component)}
  <div class="option">
    <h2>{name}</h2>
    <Comp />
  </div>
{/snippet}

<style>
  h2 {
    margin-top: 0;
    margin-bottom: 0.6rem;
  }

  #main-container {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .option {
    background-color: #282828;
    border-radius: 0.6rem;
    padding: 0.6rem;
  }
</style>

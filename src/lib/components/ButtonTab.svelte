<script lang="ts">
  import { pageTabs } from "$lib/store/status.svelte";
  import type { Component } from "svelte";

  type ButtonProps = {
    id: string;
    name: string;
    Icon: Component;
  };
  let { id, name, Icon }: ButtonProps = $props();
</script>

<button
  onclick={() => (pageTabs.downloadTab = id)}
  class={`tabs-btn ${pageTabs.downloadTab == id ? "selected" : ""}`}
>
  <Icon size={24} />
  {name}
</button>

<style>
  .tabs-btn {
    background-color: transparent;
    border-radius: 0;
    flex-direction: column;
    height: 60px;
    width: 60px;

    font-size: 0.8rem;
    font-weight: 300;
  }

  .tabs-btn.selected {
    background-color: #282828;
  }

  .tabs-btn:hover {
    background-color: #303030;
  }

  .tabs-btn:active {
    background-color: #2c2c2c;
  }
</style>

<script lang="ts">
  import type { Component } from "svelte";

  type InfoPillProps = {
    Icon: Component;
    primary: string;
    secondary?: string;
  };

  let { Icon, primary, secondary }: InfoPillProps = $props();
</script>

<div class="pill">
  <Icon size={14} strokeWidth={2} color="#FFFFFF" />
  {primary}

  {#if secondary}
    <span class="light">{secondary}</span>
  {/if}
</div>

<style>
  .pill {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 4px;

    background-color: #ffffff10;
    padding: 2px 0.5rem;
    border-radius: 1rem;

    font-size: 14px;
    font-weight: 500;
  }

  .light {
    font-weight: 300;
  }
</style>

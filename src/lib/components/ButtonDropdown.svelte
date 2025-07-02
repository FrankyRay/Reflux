<script lang="ts">
  import type { Component } from "svelte";
  import type { MouseEventHandler } from "svelte/elements";

  import Dropdown from "@lucide/svelte/icons/chevron-down";
  import { fade } from "svelte/transition";

  type ButtonProps = {
    name: string;
    onclick: MouseEventHandler<HTMLButtonElement>;
    Icon: Component;
  };
  let { name, onclick, Icon }: ButtonProps = $props();

  let showDropdown = $state(false);

  function toggleDropdown() {
    showDropdown = !showDropdown;
  }

  function handleClickOutside(event: MouseEvent) {
    const target = event.target as HTMLElement;
    if (!target.closest(".dropdown-btn")) {
      showDropdown = false;
    }
  }
</script>

<svelte:window onclick={handleClickOutside} />

<div class="main">
  <div class="main-btn">
    <button class="primary-btn" {onclick}>
      <Icon size={16} />
      {name}
    </button>
    <button class="dropdown-btn" onclick={toggleDropdown}>
      <Dropdown size={16} />
    </button>
  </div>
  {#if showDropdown}
    <div class="dropdown-opt" transition:fade={{ duration: 100 }}>
      Hello world
    </div>
  {/if}
</div>

<style>
  .main {
    position: relative;
  }

  .main-btn {
    display: flex;
    gap: 2px;
  }

  .primary-btn {
    border-top-right-radius: 0;
    border-bottom-right-radius: 0;
  }

  .dropdown-btn {
    border-top-left-radius: 0;
    border-bottom-left-radius: 0;

    padding-left: 4px;
    padding-right: 4px;
  }

  .dropdown-opt {
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
    background-color: #303030;
    border-radius: 4px;
    padding: 8px 12px;
    max-height: 100px;
    width: 300px;

    position: absolute;
    top: calc(100% + 2px);
    right: 0;
  }
</style>

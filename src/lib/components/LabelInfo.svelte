<script lang="ts">
  import Info from "@lucide/svelte/icons/info";
  import { fade } from "svelte/transition";

  let tooltipStatus = $state({
    show: false,
    x: 0,
    y: 0,
  });

  function showTooltip(event: MouseEvent) {
    tooltipStatus.show = true;
    updatePosition(event);
  }

  function moveTooltip(event: MouseEvent) {
    if (tooltipStatus.show) {
      updatePosition(event);
    }
  }

  function hideTooltip() {
    tooltipStatus.show = false;
  }

  function updatePosition(event: MouseEvent) {
    tooltipStatus.x = event.clientX + 10;
    tooltipStatus.y = event.clientY + 10;
  }

  type LabelInfo = {
    label: string;
    tooltip: string;
    onclick: () => void;
  };
  let { label, tooltip, onclick }: LabelInfo = $props();
</script>

<div class="label-container">
  <span class="label-text">{label}</span>
  <button
    class="label-info-icon"
    onmouseenter={showTooltip}
    onmousemove={moveTooltip}
    onmouseleave={hideTooltip}
    aria-label="More information"
    {onclick}
  >
    <Info size={14} />
  </button>
</div>

{#if tooltipStatus.show}
  <div
    class="label-tooltip"
    style="left: {tooltipStatus.x}px; top: {tooltipStatus.y}px;"
    transition:fade={{ duration: 100 }}
  >
    {@html tooltip}
  </div>
{/if}

<style>
  .label-container {
    display: inline-flex;
    align-items: center;
    gap: 8px;
  }

  .label-info-icon {
    cursor: pointer;
    padding: 0;
    background-color: transparent;
  }

  .label-tooltip {
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
    background-color: #303030;
    border-radius: 4px;
    padding: 8px 12px;
    max-width: 500px;
    font-size: 0.9rem;

    position: fixed;
    z-index: 1000;
    pointer-events: none;
  }

  .label-tooltip::before {
    border-left: 4px solid transparent;
    border-right: 4px solid transparent;
    border-bottom: 4px solid #333;
    content: "";

    position: absolute;
    transform: translateX(-50%);
    top: -4px;
    left: 50%;
  }
</style>

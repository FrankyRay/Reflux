<script lang="ts">
  import { progressStatus } from "$lib/store/status.svelte";
</script>

<div
  id="progress-bar"
  class={`progress-${progressStatus.status}`}
  style={`width: ${progressStatus.value}%;`}
></div>

<style>
  #progress-bar {
    background-color: transparent;
    position: absolute;
    height: 2px;

    transition:
      background-color 0.5s linear,
      width 0.5s cubic-bezier(0.37, 0, 0.63, 1);
  }

  .progress-download {
    background-color: #ffffff !important;
  }

  .progress-success {
    background-color: #c5e478 !important;
  }

  .progress-error {
    background-color: #eb9b9f !important;
  }
</style>

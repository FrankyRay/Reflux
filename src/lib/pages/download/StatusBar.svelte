<script lang="ts">
  import { downloadVideo } from "$lib/utils/download";
  import Terminal from "$lib/components/Terminal.svelte";
  import Progress from "$lib/components/Progress.svelte";
  import Download from "@lucide/svelte/icons/arrow-down-to-line";

  let consoleText = $state({
    status: "info",
    message: "Reflux v0.1.0-beta.1",
  });
</script>

<footer>
  <Progress />

  <div id="status-bar" class="container {consoleText.status}">
    <Terminal />
    <button onclick={downloadVideo} class="icon-text">
      <Download size={16} />
      Download
    </button>
  </div>
</footer>

<style>
  #status-bar {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .font-icon {
    width: 0.6rem;
    height: 0.6rem;
  }
</style>

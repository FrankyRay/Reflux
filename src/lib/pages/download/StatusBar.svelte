<script lang="ts">
  import { downloadConfig } from "$lib/store/download.svelte";
  import Download from "@lucide/svelte/icons/arrow-down-to-line";
  import { invoke } from "@tauri-apps/api/core";
  import Terminal from "$lib/components/Terminal.svelte";
  import Progress from "$lib/components/Progress.svelte";

  let consoleText = $state({
    status: "info",
    message: "Reflux v0.1.0-beta.1",
  });

  let progressBar = $state({
    status: "none",
    progress: 0,
  });

  async function downloadVideo() {
    console.log(`[DBG] Downloading video of '${downloadConfig.config?.link}'`);
    progressBar.progress = 0;
    await invoke("execute_video_download", {
      args: [
        "--newline",
        "--format",
        `${downloadConfig.config?.video}+${downloadConfig.config?.audio}`,
        "--paths",
        downloadConfig.config?.directory,
        "--output",
        downloadConfig.config?.output,
      ],
      videoLink: downloadConfig.config?.link,
    });
  }
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

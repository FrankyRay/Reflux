<script lang="ts">
  import Folder from "@lucide/svelte/icons/folder";
  import {
    downloadConfig,
    setDownloadOption,
  } from "$lib/store/download.svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import { openUrl } from "@tauri-apps/plugin-opener";
  import LabelInfo from "$lib/components/LabelInfo.svelte";

  let pathOption = $state(downloadConfig.config?.directory);
  let outputOption = $state(downloadConfig.config?.output);

  async function findDirectoryPath() {
    const path = await open({
      directory: true,
      defaultPath: pathOption,
      canCreateDirectories: true,
      title: "Find Directory Path",
    });

    if (path !== null) pathOption = path;
  }

  $effect(() => {
    setDownloadOption({
      directory: pathOption,
      output: outputOption,
    });
  });
</script>

<div id="output-select">
  <div id="path-input" class="input-box">
    <label for="path">Directory Path</label>
    <div>
      <input
        bind:value={pathOption}
        placeholder="Directory Path"
        type="text"
        name="path"
        id="path"
      />
      <button onclick={findDirectoryPath}>
        <Folder size={16} />
        Find Directory
      </button>
    </div>
  </div>

  <div id="output-input" class="input-box">
    <label for="output">
      <LabelInfo
        label="File Output"
        tooltip={`File output follow YT-DLP's output template.<br/>` +
          `Must provide <code>.%(ext)s</code> at the end for video format.<br/>` +
          `Click for more detail.`}
        onclick={() =>
          openUrl(
            "https://github.com/yt-dlp/yt-dlp?tab=readme-ov-file#output-template"
          )}
      />
    </label>
    <div>
      <input
        bind:value={outputOption}
        placeholder="%(title).200B [%(id)s].%(ext)s"
        type="text"
        name="output"
        id="output"
      />
    </div>
  </div>
</div>

<style>
  label {
    display: flex;
    align-items: center;
    gap: 4px;

    font-weight: 300;
  }

  #output-select {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .input-box > div {
    display: flex;
    gap: 8px;
  }

  .input-box > div > input {
    flex-grow: 1;
  }
</style>

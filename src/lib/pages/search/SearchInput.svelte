<script lang="ts">
  import { readText } from "@tauri-apps/plugin-clipboard-manager";
  import { openUrl } from "@tauri-apps/plugin-opener";
  import { invoke } from "@tauri-apps/api/core";

  import {
    defaultCommandOutput,
    setCommandOutput,
    pageTabs,
  } from "$lib/store/status.svelte";
  import { setDownloadOption } from "$lib/store/download.svelte";
  import { setting } from "$lib/store/settings.svelte";
  import { fastDownloadVideo } from "$lib/utils/download";

  import Button from "$lib/components/Button.svelte";
  import ButtonDropdown from "$lib/components/ButtonDropdown.svelte";

  import Clipboard from "@lucide/svelte/icons/clipboard";
  import Config from "@lucide/svelte/icons/wrench";
  import Download from "@lucide/svelte/icons/arrow-down-to-line";
  import Link from "@lucide/svelte/icons/link";

  let videoLink = $state("");

  async function configureVideo() {
    // console.log(`[DBG] Get info of video '${videoLink}'`);
    setCommandOutput({
      message: "Search video information...",
      status: "info",
    });
    try {
      let output: VideoInfo = await invoke("get_video_info", { videoLink });

      // console.log(`[DBG] Set video info`);
      // console.log(output);
      setDownloadOption({
        info: output,
        link: videoLink,
        video: "",
        audio: "",
        ...setting.output,
      });
      pageTabs.showSearch = false;
      defaultCommandOutput(0);
    } catch (err) {
      setCommandOutput({
        message: `There's something wrong! ${err}`,
        status: "error",
      });
      defaultCommandOutput();
    }
  }

  async function pasteFromClipboard() {
    videoLink = await readText();
  }

  function openSupportedLink() {
    openUrl("https://github.com/yt-dlp/yt-dlp/blob/master/supportedsites.md");
  }
</script>

<div id="search" class="container">
  <label for="video-link">Video Link</label>
  <div id="input-con">
    <input
      bind:value={videoLink}
      type="text"
      name="video-link"
      id="video-link"
      placeholder="Enter video link"
      autocomplete="off"
    />
    <Button name="Supported Link" onclick={openSupportedLink} Icon={Link} />
  </div>
  <div id="button-con">
    <div id="btn-left-con">
      <Button name="Paste" onclick={pasteFromClipboard} Icon={Clipboard} />
      <Button name="Configure" onclick={configureVideo} Icon={Config} />
      <Button
        name="Download"
        onclick={() => fastDownloadVideo(videoLink)}
        Icon={Download}
      />
    </div>
  </div>
</div>

<style>
  label {
    display: block;
  }

  #search {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  #input-con,
  #button-con,
  #btn-left-con {
    display: flex;
    gap: 8px;
  }

  #video-link {
    flex-grow: 1;
    font-weight: 700;
  }
</style>

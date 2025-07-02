<script lang="ts">
  import {
    downloadConfig,
    setDownloadOption,
  } from "$lib/store/download.svelte";
  import {
    formatBitrate,
    formatFilesize,
    formatResolution,
  } from "$lib/utils/format";
  import InfoPill from "$lib/components/InfoPill.svelte";

  import Resolution from "@lucide/svelte/icons/scaling";
  import Format from "@lucide/svelte/icons/file-video";
  import Bitrate from "@lucide/svelte/icons/audio-lines";
  import Filesize from "@lucide/svelte/icons/hard-drive";
</script>

<div id="video-select" class="selection">
  {#if downloadConfig.config !== null}
    {#each downloadConfig.config.info.formats.video.reverse() as videoFormat}
      <button
        class={`video-format ${downloadConfig.config?.video == videoFormat.id && "selected"}`}
        onclick={() => setDownloadOption({ video: videoFormat.id })}
      >
        <div class="video-id">{videoFormat.id}</div>
        <div class="empty"></div>
        <div class="video-info">
          <InfoPill
            primary={formatResolution(videoFormat.width, videoFormat.height)}
            secondary={videoFormat.note}
            Icon={Resolution}
          />
          <InfoPill
            primary={videoFormat.extension}
            secondary={videoFormat.codec}
            Icon={Format}
          />
          <InfoPill
            primary={formatBitrate(videoFormat.bitrate)}
            Icon={Bitrate}
          />
          <InfoPill
            primary={formatFilesize(videoFormat.filesize)}
            Icon={Filesize}
          />
        </div>
      </button>
    {/each}
  {/if}
</div>

<style>
  .selection {
    overflow: auto;
    display: flex;
    gap: 8px;
  }

  .video-format {
    border: 1px solid #808080;
    padding: 0;
    width: 250px;
    height: 150px;

    flex-shrink: 0;
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
  }

  .empty {
    flex-grow: 1;
  }

  .video-id {
    box-sizing: border-box;
    border-bottom: 1px dashed #808080;
    width: 100%;

    display: flex;
    padding: 3px 5px;

    font-family: var(--monospace-font);
  }

  .video-info {
    display: flex;
    align-items: flex-end;
    gap: 4px;
    flex-wrap: wrap;

    box-sizing: border-box;
    width: 100%;
    padding: 3px 5px;
  }

  .selected {
    background-color: #3e7398;
    border-color: #67bfff;
  }

  .selected > .video-id {
    border-color: #67bfff;
  }

  .selected:hover {
    background-color: #4d8fbe;
  }

  .selected:active {
    background-color: #294c65;
  }
</style>

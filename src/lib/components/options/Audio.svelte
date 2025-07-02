<script lang="ts">
  import {
    downloadConfig,
    setDownloadOption,
  } from "$lib/store/download.svelte";
  import { formatBitrate, formatFilesize } from "$lib/utils/format";
  import InfoPill from "$lib/components/InfoPill.svelte";

  import Resolution from "@lucide/svelte/icons/headphones";
  import Format from "@lucide/svelte/icons/file-audio-2";
  import Bitrate from "@lucide/svelte/icons/audio-lines";
  import Filesize from "@lucide/svelte/icons/hard-drive";
</script>

<div id="audio-select" class="selection">
  {#if downloadConfig.config !== null}
    {#each downloadConfig.config.info.formats.audio.reverse() as audioFormat}
      <button
        class={`audio-format ${downloadConfig.config?.audio == audioFormat.id && "selected"}`}
        onclick={() => setDownloadOption({ audio: audioFormat.id })}
      >
        <div class="audio-id">{audioFormat.id}</div>
        <div class="empty"></div>
        <div class="audio-info">
          <InfoPill primary={audioFormat.note} Icon={Resolution} />
          <InfoPill
            primary={audioFormat.extension}
            secondary={audioFormat.codec}
            Icon={Format}
          />
          <InfoPill
            primary={formatBitrate(audioFormat.bitrate)}
            Icon={Bitrate}
          />
          <InfoPill
            primary={formatFilesize(audioFormat.filesize)}
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

  .audio-format {
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

  .audio-id {
    box-sizing: border-box;
    border-bottom: 1px dashed #808080;
    width: 100%;

    display: flex;
    padding: 3px 5px;

    font-family: var(--monospace-font);
  }

  .audio-info {
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

  .selected > .audio-id {
    border-color: #67bfff;
  }

  .selected:hover {
    background-color: #4d8fbe;
  }

  .selected:active {
    background-color: #294c65;
  }
</style>

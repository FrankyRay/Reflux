<script lang="ts">
  import { downloadConfig } from "$lib/store/download.svelte";
  import ButtonTab from "$lib/components/ButtonTab.svelte";

  import Video from "@lucide/svelte/icons/film";
</script>

<header>
  <div id="header-container">
    <div id="video-info" class="container">
      <div id="title" class="text">{downloadConfig.config?.info.title}</div>
      <div id="uploader" class="text">
        {downloadConfig.config?.info.uploader}
      </div>
    </div>
    <ButtonTab id="output" name="Output" Icon={Video} />
  </div>
</header>

<style>
  #header-container {
    display: flex;
    align-items: center;
  }

  #video-info {
    display: flex;
    flex-direction: column;
    flex: 1;
  }

  #video-info #title {
    line-height: 1;
    font-size: 1.5rem;
    font-weight: 600;
  }

  #video-info #uploader {
    line-height: 1;
    overflow-wrap: break-word;
    font-size: 0.95rem;
    font-weight: 300;
  }

  .text {
    display: -webkit-box;
    line-clamp: 1;
    -webkit-line-clamp: 1;
    -webkit-box-orient: vertical;
    overflow: hidden;
    text-overflow: ellipsis;
  }
</style>

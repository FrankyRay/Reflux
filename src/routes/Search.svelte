<script lang="ts">
  import { fade } from "svelte/transition";

  import { ytdlpVersion } from "$lib/store/status.svelte";
  import SearchInput from "$lib/pages/search/SearchInput.svelte";
  import Terminal from "$lib/components/Terminal.svelte";
  import InfoPill from "$lib/components/InfoPill.svelte";

  import YTDLP from "@lucide/svelte/icons/terminal";
</script>

<div id="main" transition:fade={{ duration: 300 }}>
  <div id="card">
    <div id="title" class="container">
      <div id="software">Reflux</div>
      <div id="binaries">
        <InfoPill
          primary="YT-DLP"
          secondary={`v${ytdlpVersion.version}`}
          Icon={YTDLP}
        />
      </div>
    </div>
    <SearchInput />
    <div id="status" class="container">
      <Terminal />
    </div>
  </div>
</div>

<style>
  #main {
    background-color: #202020f0;
    position: absolute;
    height: 100%;
    width: 100%;
    z-index: 1;

    display: grid;
    place-items: center;
  }

  #card {
    background-color: #303030;
    border-radius: 10px;
    overflow: hidden;
    width: 60%;
  }

  #title {
    background-color: #3a5a74;
    height: 50px;
    display: flex;
    align-items: end;
  }

  #software {
    flex-grow: 1;
    font-size: 1.3rem;
    font-weight: 700;
  }

  #status {
    background-color: #404040;
    font-size: 0.9rem;
    font-family: var(--monospace-font);
  }
</style>

<script lang="ts">
  import { downloadConfig } from "$lib/store/download.svelte";
  import { pageTabs } from "$lib/store/status.svelte";
  import "$lib/store/settings.svelte";
  import "$lib/utils/shortcut";

  import Download from "./Download.svelte";
  import Search from "./Search.svelte";
</script>

{#if downloadConfig.config !== null}
  <Download />
{/if}

{#if pageTabs.showSearch || downloadConfig.config === null}
  <Search />
{/if}

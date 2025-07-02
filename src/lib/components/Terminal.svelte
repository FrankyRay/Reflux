<script lang="ts">
  import { commandOutput } from "$lib/store/status.svelte";
</script>

<div id="terminal" class={`terminal-${commandOutput.status}`}>
  {commandOutput.message}
</div>

<style>
  #terminal {
    font-family: "Geist Mono", monospace;
    text-overflow: ellipsis;
    flex: 1;
  }

  .terminal-info {
    color: #ffffff;
  }

  .terminal-warn {
    color: #ffdd7e;
  }

  .terminal-success {
    color: #c5e478;
  }

  .terminal-error {
    color: #eb9b9f;
  }
</style>

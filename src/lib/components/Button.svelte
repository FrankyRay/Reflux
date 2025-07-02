<script lang="ts">
  import type { Component } from "svelte";
  import type { MouseEventHandler } from "svelte/elements";

  type ButtonProps = {
    name: string;
    onclick: MouseEventHandler<HTMLButtonElement>;
    Icon: Component;
  };
  let { name, onclick, Icon }: ButtonProps = $props();
</script>

<button {onclick}>
  <Icon size={16} />
  {name}
</button>

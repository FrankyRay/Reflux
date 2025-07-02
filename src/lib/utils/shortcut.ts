import { invoke } from "@tauri-apps/api/core";

const FUNCTION_KEYS = ["meta", "control", "alt", "shift"];
const keyboardShortcut: Record<string, () => void> = {
  "alt+s": () => {
    invoke("open_settings_window");
  },
};

window.addEventListener("keydown", (event: KeyboardEvent) => {
  const sequences = [];

  if (event.metaKey) sequences.push("meta");
  if (event.ctrlKey) sequences.push("ctrl");
  if (event.altKey) sequences.push("alt");
  if (event.shiftKey) sequences.push("shift");
  if (!FUNCTION_KEYS.includes(event.key.toLowerCase()))
    sequences.push(event.key.toLowerCase());

  const key = sequences.join("+");
  console.log(key);

  const fn = keyboardShortcut[key];
  if (typeof fn === "function") fn();
});

export function registerShortcut(shortcut: string, fn: () => void) {
  keyboardShortcut[shortcut] = fn;
}

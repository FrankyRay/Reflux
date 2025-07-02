// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
import { listen } from "@tauri-apps/api/event";
import "$lib/styles/global.css";
import {
  defaultCommandOutput,
  defaultProgressStatus,
  setCommandOutput,
  setProgressStatus,
} from "$lib/store/status.svelte";
import { ytdlpVersion } from "$lib/store/status.svelte";
import { invoke } from "@tauri-apps/api/core";

export const prerender = true;
export const ssr = false;

(async () => {
  ytdlpVersion.version = await invoke("get_ytdlp_ver");

  await listen<StdoutPayload>("stdoutCommand", (event) => {
    const { message } = event.payload;
    console.log(message);

    if (!message.startsWith("[")) return;
    setCommandOutput({ message, status: "info" });

    if (!message.startsWith("[download]")) return;
    const progressStr = message.match(/(\d+(?:\.\d+)?)\%/g);

    if (progressStr === null) return;
    const value = Number(progressStr[0].replace("%", ""));
    setProgressStatus({ value, status: "download" });
  });

  await listen<StderrPayload>("stderrCommand", (event) => {
    const { message } = event.payload;
    console.error(message);

    if (!message.startsWith("[")) return;
    setCommandOutput({ message, status: "error" });
  });

  await listen<StatusPayload>("statusCommand", (event) => {
    const { status } = event.payload;
    console.log(`Command run with status ${status ? 0 : 1}`);

    if (status) {
      setCommandOutput({
        status: "success",
        message: "Successfully downloading video",
      });
      setProgressStatus({ status: "success", value: 100 });
    } else {
      setCommandOutput({
        status: "error",
        message: "Something wrong when downloading video",
      });
      setProgressStatus({ status: "error", value: 100 });
    }

    defaultCommandOutput();
    defaultProgressStatus();
  });
})();

import { downloadConfig } from "$lib/store/download.svelte";
import { progressStatus } from "$lib/store/status.svelte";
import { invoke } from "@tauri-apps/api/core";

export async function downloadVideo() {
  progressStatus.value = 0;

  await invoke("execute_video_download", {
    args: [
      "--newline",
      "--format",
      `${downloadConfig.config?.video}+${downloadConfig.config?.audio}`,
      "--paths",
      downloadConfig.config?.directory,
      "--output",
      downloadConfig.config?.output,
    ],
    videoLink: downloadConfig.config?.link,
  });
}

export async function fastDownloadVideo(videoLink: string) {
  await invoke("execute_video_download", {
    args: [
      "--newline",
      "--format-sort",
      "res,ext",
      "--paths",
      downloadConfig.config?.directory,
      "--output",
      downloadConfig.config?.output,
    ],
    videoLink,
  });
}

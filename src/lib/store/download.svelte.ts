type DownloadConfig = {
  link: string;
  info: VideoInfo;
  directory: string;
  output: string;
  video: string;
  audio: string;
};

export const downloadConfig = $state<{ config: DownloadConfig | null }>({
  config: null,
});

export function setDownloadOption(option: Partial<DownloadConfig>) {
  if (downloadConfig.config === null) {
    downloadConfig.config = option as DownloadConfig;
    return;
  }

  const keys = Object.keys(option) as Array<keyof DownloadConfig>;
  keys.forEach((key) => {
    const value = option[key];
    if (value !== undefined && downloadConfig.config !== null) {
      downloadConfig.config[key] = value as any;
    }
  });
}

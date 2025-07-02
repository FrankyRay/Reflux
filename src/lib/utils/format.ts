export function formatNumberSuffix(
  num: number,
  suffixes: string[],
  factor: number,
  fixed: boolean
) {
  let exponent = 0;
  while (num / factor > 1) {
    num = num / factor;
    exponent++;
  }

  const suffix =
    suffixes[exponent < suffixes.length ? exponent : suffixes.length - 1];

  if (suffix)
    return `${num >= 10 && !fixed ? num.toFixed(1) : num.toFixed(2)}${suffix}`;
  return `${num}`;
}

export function formatBitrate(bitrate: number) {
  return bitrate === 0 || bitrate === null
    ? "Unknown"
    : `${formatNumberSuffix(bitrate, [..."KMGTPEZY"], 1024, false)}bps`;
}

export function formatFilesize(filesize: number) {
  return filesize === 0 || filesize === null
    ? "Unknown"
    : `${formatNumberSuffix(filesize, ["", ..."KMGTPEZY"], 1024, true)}B`;
}

export function formatResolution(width: number, height: number) {
  return width == 0 || height == 0 ? `` : `${width}x${height}`;
}

export function formatDuration(duration: number) {
  const hour = Math.floor(duration / 3600);
  const minute = Math.floor((duration % 3600) / 60);
  const second = Math.floor(duration % 60);

  if (hour != 0)
    return `${hour.toString().padStart(2, "0")}:${minute
      .toString()
      .padStart(2, "0")}:${second.toString().padStart(2, "0")}`;
  else
    return `${minute.toString().padStart(2, "0")}:${second
      .toString()
      .padStart(2, "0")}`;
}

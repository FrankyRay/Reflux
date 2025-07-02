type AudioFormat = {
  id: string;
  note: string;
  filesize: number;
  codec: string;
  bitrate: number;
  extension: string;
};

type VideoFormat = {
  id: string;
  note: string;
  filesize: number;
  width: number;
  height: number;
  codec: string;
  bitrate: number;
  extension: string;
};

type Formats = {
  audio: AudioFormat[];
  video: VideoFormat[];
};

type VideoInfo = {
  id: string;
  title: string;
  uploader: string;
  formats: Formats;
  original: Record<string, any>;
};

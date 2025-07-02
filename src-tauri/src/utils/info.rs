use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::{shell::run_command, ytdlp::get_ytdlp};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatData {
    pub audio: Vec<AudioFormat>,
    pub video: Vec<VideoFormat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioFormat {
    pub id: String,
    pub note: String,
    pub filesize: Option<u64>,
    pub codec: String,
    pub bitrate: f64,
    pub extension: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoFormat {
    pub id: String,
    pub note: String,
    pub filesize: Option<u64>,
    pub width: u64,
    pub height: u64,
    pub codec: String,
    pub bitrate: f64,
    pub extension: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoInfo {
    pub id: String,
    pub title: String,
    pub uploader: Option<String>,
    pub formats: FormatData,
    pub original: Value,
}

pub fn get_video_data(video_link: String) -> Result<VideoInfo, String> {
    let ytdlp = get_ytdlp().unwrap();
    let dump_info = run_command(
        ytdlp,
        vec![
            "--dump-json",
            "--format-sort", "res,ext",
            &video_link,
        ],
    );

    let json_parse: Result<Value, serde_json::Error>;
    match dump_info {
        Ok(value) => json_parse = serde_json::from_str(&value),
        Err(error) => return Err(error),
    }

    let json_info: Value;
    match json_parse {
        Ok(val) => json_info = val,
        Err(_) => return Err("invalid json format".to_string()),
    }

    let format_video = parse_format(json_info["formats"].as_array().unwrap());
    Ok(VideoInfo {
        id: json_info["id"].as_str().unwrap().to_string(),
        title: json_info["title"].as_str().unwrap().to_string(),
        uploader: json_info["uploader"].as_str().map(|s| s.to_string()),
        formats: format_video,
        original: json_info,
    })
}

fn parse_format(formats: &Vec<Value>) -> FormatData {
    let mut video_formats: Vec<VideoFormat> = Vec::new();
    let mut audio_formats: Vec<AudioFormat> = Vec::new();

    for format in formats {
        if format["video_ext"].as_str().unwrap_or("none") != "none" {
            video_formats.push(VideoFormat {
                id: format["format_id"].as_str().unwrap().to_string(),
                note: format["format_note"].as_str().unwrap_or("").to_string(),
                filesize: format["filesize"].as_u64(),
                width: format["width"].as_u64().unwrap_or(0),
                height: format["height"].as_u64().unwrap_or(0),
                codec: format["vcodec"].as_str().unwrap_or("").to_string(),
                bitrate: format["vbr"].as_f64().unwrap_or(0.0),
                extension: format["video_ext"].as_str().unwrap_or("").to_string(),
            });
        } else if format["audio_ext"].as_str().unwrap_or("none") != "none" {
            audio_formats.push(AudioFormat {
                id: format["format_id"].as_str().unwrap().to_string(),
                note: format["format_note"].as_str().unwrap_or("").to_string(),
                filesize: format["filesize"].as_u64(),
                codec: format["acodec"].as_str().unwrap_or("").to_string(),
                bitrate: format["abr"].as_f64().unwrap_or(0.0),
                extension: format["audio_ext"].as_str().unwrap_or("").to_string(),
            });
        }
    }

    FormatData {
        audio: audio_formats,
        video: video_formats,
    }
}

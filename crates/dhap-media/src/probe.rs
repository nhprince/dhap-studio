//! Media metadata inspection and stream probing.

use dhap_core::{DhapError, Result, TimestampMs};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Command;

/// Metadata attributes discovered by probing a video file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MediaProbeResult {
    pub file_name: String,
    pub duration: TimestampMs,
    pub width: u32,
    pub height: u32,
    pub frame_rate: f64,
    pub format: String,
    pub codec: String,
}

/// Probes a media file using FFmpeg, returning format and stream metadata.
pub fn probe_media_file(path: &Path) -> Result<MediaProbeResult> {
    if !path.exists() {
        return Err(DhapError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Media file not found: '{}'", path.display()),
        )));
    }

    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    // Execute ffmpeg with stderr capture (ffmpeg prints stream details to stderr)
    let output = Command::new("ffmpeg")
        .arg("-hide_banner")
        .arg("-i")
        .arg(path)
        .output();

    let output = match output {
        Ok(out) => out,
        Err(e) => {
            return Err(DhapError::Media(format!(
                "Failed to execute FFmpeg. Ensure FFmpeg is installed and on system PATH. Cause: {e}"
            )));
        }
    };

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Parse Duration: 00:01:23.45,
    let duration = parse_duration(&stderr).unwrap_or(TimestampMs::from_millis(0));

    // Parse Stream ... Video: h264 (...), yuv420p, 1920x1080 [SAR ...], 30 fps
    let (width, height, frame_rate, codec) = parse_video_stream(&stderr);

    let format = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("mp4")
        .to_lowercase();

    Ok(MediaProbeResult {
        file_name,
        duration,
        width,
        height,
        frame_rate,
        format,
        codec,
    })
}

fn parse_duration(output: &str) -> Option<TimestampMs> {
    // Looks for "Duration: 00:00:10.50,"
    if let Some(pos) = output.find("Duration: ") {
        let rest = &output[pos + 10..];
        if let Some(comma) = rest.find(',') {
            let time_str = rest[..comma].trim();
            let parts: Vec<&str> = time_str.split(':').collect();
            if parts.len() == 3 {
                let hours: f64 = parts[0].parse().ok()?;
                let minutes: f64 = parts[1].parse().ok()?;
                let seconds: f64 = parts[2].parse().ok()?;
                let total_seconds = (hours * 3600.0) + (minutes * 60.0) + seconds;
                return Some(TimestampMs::from_seconds_f64(total_seconds));
            }
        }
    }
    None
}

fn parse_video_stream(output: &str) -> (u32, u32, f64, String) {
    let mut width = 1920;
    let mut height = 1080;
    let mut fps = 30.0;
    let mut codec = "h264".to_string();

    for line in output.lines() {
        if line.contains("Stream #") && line.contains("Video:") {
            // Find codec: "Video: h264 (High)" -> h264
            if let Some(v_pos) = line.find("Video: ") {
                let v_part = &line[v_pos + 7..];
                if let Some(space_pos) = v_part.find([' ', ',']) {
                    codec = v_part[..space_pos].trim().to_string();
                }
            }

            // Find resolution e.g. 1920x1080 or 1280x720
            for word in line.split([' ', ',']) {
                if let Some(x_pos) = word.find('x') {
                    let w_str = &word[..x_pos];
                    let h_str = &word[x_pos + 1..];
                    if let (Ok(w), Ok(h)) = (w_str.parse::<u32>(), h_str.parse::<u32>()) {
                        if w >= 100 && h >= 100 && w <= 7680 && h <= 4320 {
                            width = w;
                            height = h;
                            break;
                        }
                    }
                }
            }

            // Find frame rate e.g. "30 fps" or "60 fps"
            if let Some(fps_pos) = line.find(" fps") {
                let before = &line[..fps_pos];
                if let Some(space_pos) = before.rfind(' ') {
                    if let Ok(f) = before[space_pos + 1..].parse::<f64>() {
                        fps = f;
                    }
                }
            }
            break;
        }
    }

    (width, height, fps, codec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ffmpeg_output() {
        let sample = r#"
Input #0, mov,mp4,m4a,3gp,3g2,mj2, from 'sample.mp4':
  Metadata:
    major_brand     : isom
  Duration: 00:01:23.45, start: 0.000000, bitrate: 4500 kb/s
  Stream #0:0[0x1](und): Video: h264 (High) (avc1 / 0x31637661), yuv420p, 1920x1080 [SAR 1:1 DAR 16:9], 30 fps, 30 tbr
"#;
        let dur = parse_duration(sample).unwrap();
        assert_eq!(dur.as_millis(), 83450);

        let (w, h, fps, codec) = parse_video_stream(sample);
        assert_eq!(w, 1920);
        assert_eq!(h, 1080);
        assert_eq!(fps, 30.0);
        assert_eq!(codec, "h264");
    }
}

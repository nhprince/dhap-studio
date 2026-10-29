//! Extraction of representative still frames from video streams.

use dhap_core::{DhapError, Result, TimestampMs};
use image::{ImageBuffer, Rgba};
use std::fs;
use std::path::Path;
use std::process::Command;

/// Extracts a single frame at the designated timestamp offset using FFmpeg.
///
/// Uses fast seek (`-ss` before `-i`) combined with accurate frame decoding.
pub fn extract_frame_at_timestamp(
    video_path: &Path,
    timestamp: TimestampMs,
    output_image_path: &Path,
) -> Result<()> {
    if !video_path.exists() {
        return Err(DhapError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Video not found at '{}'", video_path.display()),
        )));
    }

    if let Some(parent) = output_image_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let timestamp_sec = format!("{:.3}", timestamp.as_seconds_f64());

    let status = Command::new("ffmpeg")
        .arg("-hide_banner")
        .arg("-ss")
        .arg(&timestamp_sec)
        .arg("-i")
        .arg(video_path)
        .arg("-frames:v")
        .arg("1")
        .arg("-update")
        .arg("1")
        .arg("-y")
        .arg(output_image_path)
        .output();

    match status {
        Ok(out) => {
            if !out.status.success() {
                let err_msg = String::from_utf8_lossy(&out.stderr);
                return Err(DhapError::FfmpegExecution {
                    code: out.status.code(),
                    message: format!("Frame extraction failed: {err_msg}"),
                });
            }
        }
        Err(e) => {
            return Err(DhapError::Media(format!("Failed to invoke FFmpeg: {e}")));
        }
    }

    if !output_image_path.exists() {
        return Err(DhapError::Media(format!(
            "FFmpeg reported success but output image was not created at '{}'",
            output_image_path.display()
        )));
    }

    Ok(())
}

/// Generates a synthetic test frame image directly (useful for tests and offline fixtures).
pub fn create_synthetic_frame(
    width: u32,
    height: u32,
    label: &str,
    output_path: &Path,
) -> Result<()> {
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    // Fill with a stylish slate-toned background
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = 26 + ((x * 15) / width.max(1)) as u8;
        let g = 32 + ((y * 15) / height.max(1)) as u8;
        let b = 44;
        *pixel = Rgba([r, g, b, 255]);
    }

    // Draw a prominent simulated window frame header
    for x in 0..width {
        for y in 0..40.min(height) {
            img.put_pixel(x, y, Rgba([38, 45, 60, 255]));
        }
    }

    // Draw simulated macOS/Windows style window dots
    let dots = [
        (20, 20, [239, 68, 68]),
        (36, 20, [245, 158, 11]),
        (52, 20, [34, 197, 94]),
    ];
    for (cx, cy, [r, g, b]) in dots {
        for dx in -5..=5 {
            for dy in -5..=5 {
                if dx * dx + dy * dy <= 25 {
                    let px = (cx + dx) as u32;
                    let py = (cy + dy) as u32;
                    if px < width && py < height {
                        img.put_pixel(px, py, Rgba([r, g, b, 255]));
                    }
                }
            }
        }
    }

    // Save image (format determined by extension, defaults to PNG)
    img.save(output_path)
        .map_err(|e| DhapError::Image(format!("Failed to save synthetic frame: {e}")))?;

    tracing::debug!(
        "Generated synthetic test frame '{label}' at {}",
        output_path.display()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_synthetic_frame_creation() {
        let tmp = tempdir().unwrap();
        let frame_path = tmp.path().join("frame_01.png");

        create_synthetic_frame(800, 600, "Step 1 Frame", &frame_path).unwrap();

        assert!(frame_path.exists());
        let loaded = image::open(&frame_path).unwrap();
        assert_eq!(loaded.width(), 800);
        assert_eq!(loaded.height(), 600);
    }
}

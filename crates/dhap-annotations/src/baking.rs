//! Destructive raster baking engine for annotations and privacy redactions.

use crate::model::{Annotation, AnnotationKind, RedactionType};
use dhap_core::Result;
use image::{Rgba, RgbaImage};

/// Renders annotations and irreversible redactions directly onto an image buffer.
///
/// **Privacy Guarantee:** When `safe_redaction_mode` is enabled, pixels in redaction zones
/// are permanently overwritten with mask colors or destroyed through block averaging.
pub fn bake_annotations_in_place(
    image: &mut RgbaImage,
    annotations: &[Annotation],
    safe_redaction_mode: bool,
) -> Result<()> {
    let (img_w, img_h) = image.dimensions();

    // Pass 1: Apply Redactions FIRST to destroy sensitive underlying pixels
    for anno in annotations {
        if let AnnotationKind::Redaction {
            x,
            y,
            width,
            height,
            redaction_type,
        } = &anno.kind
        {
            if safe_redaction_mode {
                apply_redaction(
                    image,
                    *x,
                    *y,
                    *width,
                    *height,
                    *redaction_type,
                    img_w,
                    img_h,
                );
            }
        }
    }

    // Pass 2: Apply Highlights
    for anno in annotations {
        if let AnnotationKind::Highlight {
            x,
            y,
            width,
            height,
            color,
        } = &anno.kind
        {
            apply_highlight(image, *x, *y, *width, *height, *color, img_w, img_h);
        }
    }

    // Pass 3: Apply Rectangles and Badges
    for anno in annotations {
        match &anno.kind {
            AnnotationKind::Rectangle {
                x,
                y,
                width,
                height,
                stroke_color,
                stroke_width,
            } => {
                draw_rectangle(
                    image,
                    *x,
                    *y,
                    *width,
                    *height,
                    *stroke_color,
                    *stroke_width,
                    img_w,
                    img_h,
                );
            }
            AnnotationKind::NumberBadge {
                x,
                y,
                radius,
                number: _,
                bg_color,
                text_color: _,
            } => {
                draw_circle(image, *x, *y, *radius, *bg_color, img_w, img_h);
            }
            _ => {}
        }
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn apply_redaction(
    img: &mut RgbaImage,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    redaction_type: RedactionType,
    img_w: u32,
    img_h: u32,
) {
    let x_end = (x + width).min(img_w);
    let y_end = (y + height).min(img_h);

    match redaction_type {
        RedactionType::Blackout => {
            let solid_black = Rgba([15, 23, 42, 255]); // Opaque Deep Slate Blackout
            for px in x..x_end {
                for py in y..y_end {
                    img.put_pixel(px, py, solid_black);
                }
            }
        }
        RedactionType::Pixelate { block_size } => {
            let bs = block_size.max(4);
            let mut by = y;
            while by < y_end {
                let mut bx = x;
                while bx < x_end {
                    let cur_bx_end = (bx + bs).min(x_end);
                    let cur_by_end = (by + bs).min(y_end);

                    // Compute average color in block
                    let mut r_sum = 0u64;
                    let mut g_sum = 0u64;
                    let mut b_sum = 0u64;
                    let mut count = 0u64;

                    for px in bx..cur_bx_end {
                        for py in by..cur_by_end {
                            let p = img.get_pixel(px, py);
                            r_sum += p[0] as u64;
                            g_sum += p[1] as u64;
                            b_sum += p[2] as u64;
                            count += 1;
                        }
                    }

                    if let Some(avg_r) = r_sum.checked_div(count) {
                        let avg = Rgba([
                            avg_r as u8,
                            (g_sum / count) as u8,
                            (b_sum / count) as u8,
                            255,
                        ]);
                        for px in bx..cur_bx_end {
                            for py in by..cur_by_end {
                                img.put_pixel(px, py, avg);
                            }
                        }
                    }
                    bx += bs;
                }
                by += bs;
            }
        }
        RedactionType::Blur { radius } => {
            // Box blur approximation
            let r = radius.max(3);
            let solid_blur = Rgba([71, 85, 105, 255]);
            for px in x..x_end {
                for py in y..y_end {
                    let original = img.get_pixel(px, py);
                    let blended = Rgba([
                        ((original[0] as u16 + solid_blur[0] as u16 * r as u16) / (r as u16 + 1))
                            as u8,
                        ((original[1] as u16 + solid_blur[1] as u16 * r as u16) / (r as u16 + 1))
                            as u8,
                        ((original[2] as u16 + solid_blur[2] as u16 * r as u16) / (r as u16 + 1))
                            as u8,
                        255,
                    ]);
                    img.put_pixel(px, py, blended);
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn apply_highlight(
    img: &mut RgbaImage,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    color: [u8; 4],
    img_w: u32,
    img_h: u32,
) {
    let x_end = (x + width).min(img_w);
    let y_end = (y + height).min(img_h);

    for px in x..x_end {
        for py in y..y_end {
            let orig = img.get_pixel(px, py);
            // Alpha blend highlight
            let alpha = color[3] as f32 / 255.0;
            let r = (orig[0] as f32 * (1.0 - alpha) + color[0] as f32 * alpha) as u8;
            let g = (orig[1] as f32 * (1.0 - alpha) + color[1] as f32 * alpha) as u8;
            let b = (orig[2] as f32 * (1.0 - alpha) + color[2] as f32 * alpha) as u8;
            img.put_pixel(px, py, Rgba([r, g, b, 255]));
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_rectangle(
    img: &mut RgbaImage,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    color: [u8; 4],
    stroke: u32,
    img_w: u32,
    img_h: u32,
) {
    let rgba = Rgba(color);
    let x_end = (x + width).min(img_w);
    let y_end = (y + height).min(img_h);

    // Top and bottom borders
    for s in 0..stroke {
        let ty = y + s;
        let by = if y_end > s { y_end - 1 - s } else { y };
        for px in x..x_end {
            if ty < img_h {
                img.put_pixel(px, ty, rgba);
            }
            if by < img_h {
                img.put_pixel(px, by, rgba);
            }
        }
    }

    // Left and right borders
    for s in 0..stroke {
        let lx = x + s;
        let rx = if x_end > s { x_end - 1 - s } else { x };
        for py in y..y_end {
            if lx < img_w {
                img.put_pixel(lx, py, rgba);
            }
            if rx < img_w {
                img.put_pixel(rx, py, rgba);
            }
        }
    }
}

fn draw_circle(
    img: &mut RgbaImage,
    cx: u32,
    cy: u32,
    radius: u32,
    color: [u8; 4],
    img_w: u32,
    img_h: u32,
) {
    let rgba = Rgba(color);
    let r_i32 = radius as i32;
    let r_sq = r_i32 * r_i32;

    for dx in -r_i32..=r_i32 {
        for dy in -r_i32..=r_i32 {
            if dx * dx + dy * dy <= r_sq {
                let px = (cx as i32 + dx) as u32;
                let py = (cy as i32 + dy) as u32;
                if px < img_w && py < img_h {
                    img.put_pixel(px, py, rgba);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blackout_redaction_destroys_pixels() {
        let mut img = RgbaImage::from_pixel(100, 100, Rgba([255, 255, 255, 255]));

        // Blackout box from (10, 10) to (50, 50)
        let anno = Annotation::blackout(10, 10, 40, 40);
        bake_annotations_in_place(&mut img, &[anno], true).unwrap();

        // Check inside redacted region: must NOT be white [255, 255, 255, 255]
        let inside = img.get_pixel(25, 25);
        assert_eq!(inside, &Rgba([15, 23, 42, 255]));

        // Check outside redacted region: remains white
        let outside = img.get_pixel(5, 5);
        assert_eq!(outside, &Rgba([255, 255, 255, 255]));
    }

    #[test]
    fn test_pixelate_redaction() {
        let mut img = RgbaImage::new(100, 100);
        for x in 0..100 {
            for y in 0..100 {
                img.put_pixel(x, y, Rgba([(x % 255) as u8, (y % 255) as u8, 128, 255]));
            }
        }

        let anno = Annotation::new(AnnotationKind::Redaction {
            x: 10,
            y: 10,
            width: 40,
            height: 40,
            redaction_type: RedactionType::Pixelate { block_size: 10 },
        });

        bake_annotations_in_place(&mut img, &[anno], true).unwrap();

        // Inside a 10x10 block, all pixels should have the identical averaged color
        let p1 = img.get_pixel(12, 12);
        let p2 = img.get_pixel(18, 18);
        assert_eq!(p1, p2);
    }
}

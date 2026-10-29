//! Vector annotation models and geometry definitions.

use dhap_core::Id;
use serde::{Deserialize, Serialize};

/// Type of redaction to apply to sensitive screen regions.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum RedactionType {
    /// Irreversible solid blackout mask (recommended for guaranteed privacy).
    #[default]
    Blackout,
    /// Pixelation block averaging.
    Pixelate { block_size: u32 },
    /// Box blur filter.
    Blur { radius: u32 },
}

/// An individual visual annotation or redaction layer attached to a frame.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AnnotationKind {
    Rectangle {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        stroke_color: [u8; 4],
        stroke_width: u32,
    },
    NumberBadge {
        x: u32,
        y: u32,
        radius: u32,
        number: u32,
        bg_color: [u8; 4],
        text_color: [u8; 4],
    },
    Arrow {
        start_x: u32,
        start_y: u32,
        end_x: u32,
        end_y: u32,
        color: [u8; 4],
        stroke_width: u32,
    },
    Highlight {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        color: [u8; 4],
    },
    TextCallout {
        x: u32,
        y: u32,
        text: String,
        bg_color: [u8; 4],
        text_color: [u8; 4],
    },
    Redaction {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        redaction_type: RedactionType,
    },
}

/// Container struct wrapping an annotation with its stable ID and label.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Annotation {
    pub id: Id,
    pub kind: AnnotationKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl Annotation {
    #[must_use]
    pub fn new(kind: AnnotationKind) -> Self {
        Self {
            id: Id::with_prefix("anno"),
            kind,
            label: None,
        }
    }

    /// Helper constructor for a solid blackout redaction mask.
    #[must_use]
    pub fn blackout(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self::new(AnnotationKind::Redaction {
            x,
            y,
            width,
            height,
            redaction_type: RedactionType::Blackout,
        })
    }

    /// Helper constructor for a numbered step callout badge.
    #[must_use]
    pub fn number_badge(x: u32, y: u32, number: u32) -> Self {
        Self::new(AnnotationKind::NumberBadge {
            x,
            y,
            radius: 16,
            number,
            bg_color: [225, 29, 72, 255], // Crimson accent
            text_color: [255, 255, 255, 255],
        })
    }

    /// Returns true if this annotation represents a privacy redaction.
    #[must_use]
    pub fn is_redaction(&self) -> bool {
        matches!(self.kind, AnnotationKind::Redaction { .. })
    }
}

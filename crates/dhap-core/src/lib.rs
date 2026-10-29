//! # dhap-core
//!
//! Core primitives, strong identifiers, rational timestamps, and common error types
//! for the Dhap (ধাপ) documentation studio.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

pub mod error;
pub use error::{DhapError, Result};

/// A strongly-typed unique identifier used across Dhap entities.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Id(String);

impl Id {
    /// Generates a new random UUID v4 identifier.
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    /// Generates a new identifier with a human-readable prefix (e.g. `step_`, `frame_`).
    #[must_use]
    pub fn with_prefix(prefix: &str) -> Self {
        let clean = Uuid::new_v4().simple().to_string();
        Self(format!("{prefix}_{clean}"))
    }

    /// Creates an identifier from a string slice without generating a new UUID.
    #[must_use]
    pub fn from_string(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    /// Returns the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Id {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for Id {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// A media timestamp represented as integer milliseconds from media start.
///
/// Using integer milliseconds avoids floating-point precision drift across
/// video frames, UI timelines, and export generators.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
#[serde(transparent)]
pub struct TimestampMs(pub u64);

impl TimestampMs {
    /// Zero timestamp offset.
    pub const ZERO: Self = Self(0);

    /// Constructs a timestamp from milliseconds.
    #[must_use]
    pub const fn from_millis(ms: u64) -> Self {
        Self(ms)
    }

    /// Constructs a timestamp from fractional seconds.
    #[must_use]
    pub fn from_seconds_f64(secs: f64) -> Self {
        if secs <= 0.0 {
            Self(0)
        } else {
            Self((secs * 1000.0).round() as u64)
        }
    }

    /// Returns the raw millisecond value.
    #[must_use]
    pub const fn as_millis(&self) -> u64 {
        self.0
    }

    /// Returns the timestamp as seconds (floating-point).
    #[must_use]
    pub fn as_seconds_f64(&self) -> f64 {
        self.0 as f64 / 1000.0
    }

    /// Formats the timestamp as `MM:SS`.
    #[must_use]
    pub fn format_mm_ss(&self) -> String {
        let total_seconds = self.0 / 1000;
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        format!("{minutes:02}:{seconds:02}")
    }

    /// Formats the timestamp as `HH:MM:SS.mmm`.
    #[must_use]
    pub fn format_hh_mm_ss_mmm(&self) -> String {
        let millis = self.0 % 1000;
        let total_seconds = self.0 / 1000;
        let seconds = total_seconds % 60;
        let minutes = (total_seconds / 60) % 60;
        let hours = total_seconds / 3600;
        format!("{hours:02}:{minutes:02}:{seconds:02}.{millis:03}")
    }
}

impl fmt::Display for TimestampMs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_mm_ss())
    }
}

/// Helper function returning current UTC time.
#[must_use]
pub fn now_utc() -> DateTime<Utc> {
    Utc::now()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_generation() {
        let id1 = Id::new();
        let id2 = Id::new();
        assert_ne!(id1, id2);
        assert!(!id1.as_str().is_empty());

        let step_id = Id::with_prefix("step");
        assert!(step_id.as_str().starts_with("step_"));
    }

    #[test]
    fn test_timestamp_arithmetic_and_formatting() {
        let t0 = TimestampMs::ZERO;
        assert_eq!(t0.as_millis(), 0);
        assert_eq!(t0.format_mm_ss(), "00:00");
        assert_eq!(t0.format_hh_mm_ss_mmm(), "00:00:00.000");

        let t1 = TimestampMs::from_seconds_f64(65.432);
        assert_eq!(t1.as_millis(), 65432);
        assert_eq!(t1.format_mm_ss(), "01:05");
        assert_eq!(t1.format_hh_mm_ss_mmm(), "00:01:05.432");

        let t2 = TimestampMs::from_millis(3661005);
        assert_eq!(t2.format_hh_mm_ss_mmm(), "01:01:01.005");
    }

    #[test]
    fn test_timestamp_ordering() {
        let a = TimestampMs::from_millis(100);
        let b = TimestampMs::from_millis(200);
        assert!(a < b);
    }
}

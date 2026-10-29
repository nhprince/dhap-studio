//! # dhap-media
//!
//! Media probing, frame extraction via FFmpeg, and scene heuristics for Dhap.

pub mod extract;
pub mod heuristics;
pub mod probe;

pub use extract::{create_synthetic_frame, extract_frame_at_timestamp};
pub use heuristics::{propose_keyframes_by_interval, KeyframeProposal};
pub use probe::{probe_media_file, MediaProbeResult};

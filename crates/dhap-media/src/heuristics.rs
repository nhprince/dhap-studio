//! Keyframe proposal heuristics and scene change candidate detection.

use dhap_core::TimestampMs;
use serde::{Deserialize, Serialize};

/// A proposed keyframe candidate for the user's editorial review.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KeyframeProposal {
    pub timestamp: TimestampMs,
    pub reason: String,
    pub confidence_hint: f32,
}

/// Generates candidate step timestamps across a media duration based on interval heuristics.
#[must_use]
pub fn propose_keyframes_by_interval(
    duration: TimestampMs,
    interval: TimestampMs,
) -> Vec<KeyframeProposal> {
    let mut proposals = Vec::new();
    let total_ms = duration.as_millis();
    let interval_ms = interval.as_millis().max(1000); // minimum 1 second interval

    let mut current_ms = 0;
    let mut step_count = 1;

    while current_ms < total_ms {
        let reason = if current_ms == 0 {
            "Initial setup screen".to_string()
        } else {
            format!("Workflow checkpoint {step_count}")
        };

        proposals.push(KeyframeProposal {
            timestamp: TimestampMs::from_millis(current_ms),
            reason,
            confidence_hint: 0.85,
        });

        current_ms += interval_ms;
        step_count += 1;
    }

    proposals
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval_proposals() {
        let dur = TimestampMs::from_seconds_f64(30.0);
        let interval = TimestampMs::from_seconds_f64(10.0);

        let proposals = propose_keyframes_by_interval(dur, interval);
        assert_eq!(proposals.len(), 3);
        assert_eq!(proposals[0].timestamp.as_millis(), 0);
        assert_eq!(proposals[1].timestamp.as_millis(), 10000);
        assert_eq!(proposals[2].timestamp.as_millis(), 20000);
    }
}

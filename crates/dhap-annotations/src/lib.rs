//! # dhap-annotations
//!
//! Vector annotation geometry, layer persistence, and irreversible pixel-level redaction
//! baking for Dhap.

pub mod baking;
pub mod model;

pub use baking::bake_annotations_in_place;
pub use model::{Annotation, AnnotationKind, RedactionType};

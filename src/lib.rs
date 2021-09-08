#![crate_type = "lib"]

/// The traits defined by this crate.
mod traits;

/// The dot structure.
mod dot;

/// The vector clock implementation.
mod vclock;

/// Re-export the internal structures.
pub use crate::{dot::Dot, traits::CmRDT, traits::CvRDT, vclock::VClock};

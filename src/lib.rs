#![crate_type = "lib"]

/// The traits defined by this crate.
mod traits;

/// The dot structure.
mod dot;

/// Re-export the internal structures.
pub use crate::{dot::Dot, traits::CmRDT, traits::CvRDT};

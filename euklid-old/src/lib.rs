//! A rust library for CRDTs

#![crate_type = "lib"]
#![deny(missing_docs)]
#![deny(unreachable_pub)]

/// The traits defined by this crate.
mod traits;

/// The dot structure.
mod dot;

/// The vector clock implementation.
mod vclock;

/// The grow-counter implementation.
mod gcounter;

/// The pncounter implementation.
mod pncounter;

/// Re-export the internal structures.
pub use crate::{
    dot::Dot, gcounter::GCounter, pncounter::Op, pncounter::PNCounter, traits::CmRDT,
    traits::CvRDT, vclock::VClock,
};

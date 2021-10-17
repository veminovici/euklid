//! A rust library for CRDT clocks.

#![deny(missing_docs)]
#![deny(unreachable_pub)]

mod causalord;
mod dot;
mod dvv;
mod gcounter;
mod pncounter;
mod vclock;

/// An operation based CRDT.
/// Such CRDTs replicate by transmiting each operation.
pub trait CmRDT {
    /// The operation applied to the CRDT
    type Op;

    /// Apply an operation to the CRDT
    fn apply_op(&mut self, op: Self::Op);
}

/// A state based CRDT.
/// Such CRDTs replicate by transmitting the entire CRDT state.
pub trait CvRDT {
    /// Merge the given CRDT into the current CRDT
    fn merge(&mut self, other: Self);
}

pub use crate::causalord::*;
pub use crate::dot::Dot;
pub use crate::dvv::Dvv;
pub use crate::gcounter::GCounter;
pub use crate::pncounter::PnCounter;
pub use crate::vclock::VClock;

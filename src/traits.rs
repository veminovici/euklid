use std::error::Error;
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// Represents the identity value `zero`.
pub trait Zero {
    /// Returns the `zero` value for the type.
    fn zero() -> Self;
}

/// represents the identity value `one`.
pub trait One {
    /// Returns the `one` value for the type.
    fn one() -> Self;
}

/// The counter for the `dot` and `vclock` structures
pub trait Counter:
    Copy + PartialOrd + Add<Output = Self> + AddAssign + Sub<Output = Self> + SubAssign + Zero + One
{
    /// Returns a counter with an incremented value.
    fn incr(&self) -> Self {
        *self + Self::one()
    }

    /// Increments in-place the counter.
    fn incr_assign(&mut self) {
        *self += Self::one()
    }

    /// Returns a counter with a decremented value.
    fn decr(&self) -> Self {
        *self - Self::one()
    }

    /// Decrements in-place the counter.
    fn decr_assign(&mut self) {
        *self -= Self::one()
    }
}

/// The actor identifier.
pub trait Actor: Copy + Ord + Zero {}

/// The set assignement operation.
pub trait UpdateAssign<Rhs = Self> {
    /// Performs the update operation.
    fn upd_assign(&mut self, rhs: Rhs);
}

/// Represents the potential causality relations between two given events.
#[repr(u8)]
#[derive(PartialEq, Debug)]
pub enum Causality {
    /// An event precedes another event.
    Precede,
    /// An event is equal with another event.
    Equal,
    /// An event succeeds another event.
    Succeed,
    /// An event is concurrent with another event.
    Concurrent,
}

/// A trait that compares two events and returns their causality relation.
pub trait CausalityOrd: PartialOrd {
    /// Returns the causality relation between two entities.
    fn causality_cmp(&self, other: &Self) -> Causality {
        match self.partial_cmp(other) {
            Some(core::cmp::Ordering::Equal) => Causality::Equal,
            Some(core::cmp::Ordering::Less) => Causality::Precede,
            Some(core::cmp::Ordering::Greater) => Causality::Succeed,
            None => Causality::Concurrent,
        }
    }
}

/// Converge or state based CRDT's replicate by transmitting the entire CRDT state.
pub trait CvRDT {
    /// The validation error returned by `validate_merge`.
    type Validation: Error;

    /// Determines if a merge operation should be done.
    fn validate_merge(&self, other: &Self) -> Result<(), Self::Validation>;

    /// Merge the given CRDT into the current CRDT.
    fn merge(&mut self, other: Self);
}

/// Commutative or operation based CRDT's replicate by transmitting each operation.
pub trait CmRDT {
    /// Op's must be idempotent, meaning any Op may be applied more than once.
    type Op;

    /// The validation error returned by `validate_op`.
    type Validation: Error;

    /// Determines if the operation should be applied.
    fn validate_op(&self, op: &Self::Op) -> Result<(), Self::Validation>;

    /// Apply an Op to the CRDT
    fn apply(&mut self, op: Self::Op);
}

/// CRDT value
pub trait CRDT {
    /// The type of the value.
    type Output;

    /// Returns the value of the CRDT.
    fn value(&self) -> Self::Output;
}

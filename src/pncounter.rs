use std::fmt::{Debug, Display};

use crate::{Actor, CmRDT, Counter, CvRDT, Dot, GCounter, CRDT};

/// PNCounter
pub struct PNCounter<A: Actor, C: Counter> {
    pdots: GCounter<A, C>,
    ndots: GCounter<A, C>,
}

/// Indicates if the operation
pub enum PNOperation<A: Actor, C: Counter> {
    /// Positive operation
    Pos(Dot<A, C>),
    /// Negative operation
    Neg(Dot<A, C>),
}

//
// Constructors
//

impl<A: Actor, C: Counter> Default for PNCounter<A, C> {
    fn default() -> Self {
        Self {
            pdots: Default::default(),
            ndots: Default::default(),
        }
    }
}

//
// CvRDT, CmRDT, CRDT
//

impl<A: Actor + Debug + Display, C: Counter + Debug + Display> CmRDT for PNCounter<A, C> {
    type Op = PNOperation<A, C>;
    type Validation = <GCounter<A, C> as CmRDT>::Validation;

    fn validate_op(&self, op: &Self::Op) -> Result<(), Self::Validation> {
        match op {
            PNOperation::Pos(dot) => self.pdots.validate_op(dot),
            PNOperation::Neg(dot) => self.ndots.validate_op(dot),
        }
    }

    fn apply(&mut self, op: Self::Op) {
        match op {
            PNOperation::Pos(dot) => self.pdots.apply(dot),
            PNOperation::Neg(dot) => self.ndots.apply(dot),
        }
    }
}

impl<A: Actor + Debug + Display, C: Counter + Debug + Display> CvRDT for PNCounter<A, C> {
    type Validation = <GCounter<A, C> as CvRDT>::Validation;

    fn validate_merge(&self, other: &Self) -> Result<(), Self::Validation> {
        self.pdots.validate_merge(&other.pdots)?;
        self.ndots.validate_merge(&other.ndots)
    }

    fn merge(&mut self, other: Self) {
        self.pdots.merge(other.pdots);
        self.ndots.merge(other.ndots);
    }
}

impl<A: Actor, C: Counter + Into<usize>> CRDT for PNCounter<A, C> {
    type Output = usize;

    fn value(&self) -> Self::Output {
        let p = self.pdots.value();
        let n = self.ndots.value();
        p - n
    }
}

//
// Implementation
//

impl<A: Actor, C: Counter> PNCounter<A, C> {
    /// Constructs a new `PNCounter` instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Increments in-place the counter.
    pub fn incr_assign(&mut self, actor: &A) {
        self.pdots.incr_assign(actor);
    }

    /// Decrements in-place the counter.
    pub fn decr_assign(&mut self, actor: &A) {
        self.ndots.incr_assign(actor);
    }
}

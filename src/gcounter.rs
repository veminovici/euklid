use std::convert::Infallible;
use std::fmt::{Debug, Display};

use crate::Actor;
use crate::CmRDT;
use crate::Counter;
use crate::CvRDT;
use crate::Dot;
use crate::VClock;
use crate::CRDT;

/// A growing counter.
pub struct GCounter<A: Actor, C: Counter> {
    vclock: VClock<A, C>,
}

//
// Constructors
//

impl<A: Actor, C: Counter> Default for GCounter<A, C> {
    fn default() -> Self {
        Self {
            vclock: Default::default(),
        }
    }
}

impl<A: Actor, C: Counter> From<Vec<(A, C)>> for GCounter<A, C> {
    fn from(pairs: Vec<(A, C)>) -> Self {
        Self {
            vclock: pairs.into(),
        }
    }
}

//
// CvRDT, CmRDT, CRDT
//

impl<A: Actor + Debug + Display, C: Counter + Debug + Display> CvRDT for GCounter<A, C> {
    type Validation = Infallible;

    fn validate_merge(&self, _: &Self) -> Result<(), Self::Validation> {
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        self.vclock.merge(other.vclock);
    }
}

impl<A: Actor + Debug + Display, C: Counter + Debug + Display> CmRDT for GCounter<A, C> {
    type Op = Dot<A, C>;
    type Validation = Infallible;

    fn validate_op(&self, _: &Self::Op) -> Result<(), Self::Validation> {
        Ok(())
    }

    fn apply(&mut self, op: Self::Op) {
        self.vclock.apply(op);
    }
}

impl<A: Actor, C: Counter + Into<usize>> CRDT for GCounter<A, C> {
    type Output = usize;

    fn value(&self) -> Self::Output {
        self.vclock
            .iter()
            .map(|(_, counter)| {
                let c: usize = counter.into();
                c
            })
            .sum()
    }
}

//
// Implementation
//

impl<A: Actor, C: Counter> GCounter<A, C> {
    /// Constructs a new `GCounter` instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Increments in-place the counter for an actor.
    pub fn incr_assign(&mut self, actor: &A) {
        self.vclock.incr_assign(actor)
    }
}

//
// Tests
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_pairs() {
        let gc: GCounter<i8, usize> = vec![(1, 10), (2, 20), (3, 30)].into();
        assert_eq!(10 + 20 + 30, gc.value());
    }

    #[test]
    fn incr_assign() {
        let mut gc = GCounter::<i8, usize>::new();
        gc.incr_assign(&10);
        gc.incr_assign(&20);
        gc.incr_assign(&20);

        assert_eq!(3, gc.value());
    }

    #[test]
    fn merge() {
        let mut gc1 = GCounter::<i8, usize>::new();
        gc1.incr_assign(&10);
        gc1.incr_assign(&20);
        gc1.incr_assign(&20);

        let mut gc2 = GCounter::<i8, usize>::new();
        gc2.incr_assign(&20);
        gc2.incr_assign(&30);
        gc2.incr_assign(&30);
        gc2.incr_assign(&30);

        gc1.merge(gc2);

        assert_eq!(1 + 2 + 3, gc1.value());
    }

    #[test]
    fn apply() {
        let mut gc1 = GCounter::<i8, usize>::new();
        gc1.incr_assign(&10);
        gc1.incr_assign(&20);
        gc1.incr_assign(&20);

        gc1.apply(Dot::new(10, 3));

        assert_eq!(3 + 2, gc1.value());
    }
}

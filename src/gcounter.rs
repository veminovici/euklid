use core::fmt::{Debug, Display};

use crate::{CmRDT, CvRDT, Dot, VClock};

/// A grow-only counter
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GCounter<A: Ord> {
    /// dots store the dots, one for each actor
    dots: VClock<A>,
}

impl<A: Ord> Default for GCounter<A> {
    fn default() -> Self {
        Self {
            dots: Default::default(),
        }
    }
}

impl<A: Ord + Display> Display for GCounter<A> {
    /// Formats the display string
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<gcounter")?;
        write!(f, "{}", self.dots)?;
        write!(f, ">")
    }
}

//
// CRDT
//

impl<A: Ord + Clone + Debug> CmRDT for GCounter<A> {
    type Op = Dot<A>;

    fn apply(&mut self, op: Self::Op) {
        self.dots.apply(op)
    }
}

impl<A: Ord + Clone + Debug> CvRDT for GCounter<A> {
    fn merge(&mut self, other: Self) {
        self.dots.merge(other.dots);
    }
}

//
// API
//

impl<A: Ord + Clone> GCounter<A> {
    /// Build a new grow-counter
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns the op for incrementing the counter for a given actor
    pub fn inc_op(&self, actor: A) -> Dot<A> {
        self.dots.inc_op(actor)
    }

    /// Returns the op for increasing the counter for a given actor
    pub fn step_op(&self, actor: A, s: u64) -> Dot<A> {
        self.dots.step_op(actor, s)
    }

    /// Returns the counter value
    pub fn counter(&self) -> u64 {
        self.dots.iter().map(|dot| dot.counter).sum()
    }
}

//
// TEST
//

#[cfg(test)]
mod utest {
    use super::*;

    #[test]
    fn test_display() {
        let mut a = GCounter::new();
        a.apply(a.inc_op("A"));
        eprintln!("{}", a);
    }

    #[test]
    fn test_debug() {
        let mut a = GCounter::new();
        a.apply(a.inc_op("A"));
        eprintln!("{:?}", a);
    }

    #[test]
    fn test_merge() {
        let mut a = GCounter::new();
        a.apply(a.inc_op("A"));
        let mut b = GCounter::new();
        b.apply(b.inc_op("B"));

        a.merge(b);
        assert_eq!(a.counter(), 2);
    }

    #[test]
    fn test_basic_by_one() {
        let mut a = GCounter::new();
        let mut b = GCounter::new();
        a.apply(a.inc_op("A"));
        b.apply(b.inc_op("B"));

        assert_eq!(a.counter(), b.counter());
        assert_ne!(a, b);

        a.apply(a.inc_op("A"));
        assert_eq!(a.counter(), b.counter() + 1);
    }

    #[test]
    fn test_basic_by_step() {
        let mut a = GCounter::new();
        let mut b = GCounter::new();
        a.apply(a.step_op("A", 3));
        b.apply(b.step_op("B", 3));

        assert_eq!(a.counter(), 3);
        assert_eq!(a.counter(), b.counter());
        assert_ne!(a, b);

        a.apply(a.inc_op("A"));
        assert_eq!(a.counter(), b.counter() + 1);
    }
}

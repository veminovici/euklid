use core::fmt::{Debug, Display};

use crate::{CmRDT, CvRDT, Dot, GCounter};

/// A pn-counter
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PNCounter<A: Ord> {
    /// the increment operations.
    p: GCounter<A>,
    /// the decrement operations.
    n: GCounter<A>,
}

impl<A: Ord> Default for PNCounter<A> {
    fn default() -> Self {
        Self {
            p: Default::default(),
            n: Default::default(),
        }
    }
}

/// The operations that can take place on a pncounter
pub enum Op<A: Ord> {
    /// Increment operation
    Increment(Dot<A>),
    /// Decrement operation
    Decrement(Dot<A>),
}

//
// Display, Debug
//

impl<A: Ord + Display> Display for PNCounter<A> {
    /// Formats the display string
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<gcounter")?;
        write!(f, "{}", self.p)?;
        write!(f, "{}", self.n)?;
        write!(f, ">")
    }
}

//
// CRDT
//

impl<A: Ord + Clone + Debug> CmRDT for PNCounter<A> {
    type Op = Op<A>;

    fn apply(&mut self, op: Self::Op) {
        match op {
            Op::Increment(dot) => self.p.apply(dot),
            Op::Decrement(dot) => self.n.apply(dot),
        }
    }
}

impl<A: Ord + Clone + Debug> CvRDT for PNCounter<A> {
    fn merge(&mut self, other: Self) {
        self.p.merge(other.p);
        self.n.merge(other.n);
    }
}

//
// API
//

impl<A: Ord + Clone + Debug> PNCounter<A> {
    /// Builds a new pncounter
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates an increment operation
    pub fn inc_op(&self, actor: A) -> Op<A> {
        Op::Increment(self.p.inc_op(actor))
    }

    /// Creates a decrement operation
    pub fn decr_op(&self, actor: A) -> Op<A> {
        Op::Decrement(self.n.inc_op(actor))
    }

    /// Create an increase operation
    pub fn stepup_op(&self, actor: A, step: u64) -> Op<A> {
        Op::Increment(self.p.step_op(actor, step))
    }

    /// Create a decrease operation
    pub fn stepdown_op(&self, actor: A, step: u64) -> Op<A> {
        Op::Decrement(self.n.step_op(actor, step))
    }

    /// Increment the value of the counter
    pub fn inc(&mut self, actor: A) {
        self.apply(self.inc_op(actor))
    }

    /// Decrement the value of the counter
    pub fn decr(&mut self, actor: A) {
        self.apply(self.decr_op(actor))
    }

    /// Increases the vlaue of the counter
    pub fn step_up(&mut self, actor: A, step: u64) {
        self.apply(self.stepup_op(actor, step))
    }

    /// Decreases the value of the counter
    pub fn step_down(&mut self, actor: A, step: u64) {
        self.apply(self.stepdown_op(actor, step))
    }

    /// Get the counter value
    pub fn counter(&self) -> u64 {
        let p = self.p.counter();
        let n = self.n.counter();
        p - n
    }
}

//
// TESTS
//

#[cfg(test)]
mod utest {
    use super::*;

    #[test]
    fn test_display() {
        let mut a = PNCounter::new();
        a.apply(a.inc_op("A"));
        eprintln!("{}", a);
    }

    #[test]
    fn test_debug() {
        let mut a = PNCounter::new();
        a.apply(a.inc_op("A"));
        eprintln!("{:?}", a);
    }

    #[test]
    fn test_apply() {
        let mut a = PNCounter::new();
        a.apply(a.inc_op("A"));
        a.apply(a.stepup_op("A", 5));
        a.apply(a.decr_op("A"));
        a.apply(a.stepdown_op("A", 2));

        assert_eq!(a.counter(), 3);
    }

    #[test]
    fn test_merge() {
        let mut a = PNCounter::new();
        a.apply(a.inc_op("A"));
        a.apply(a.stepup_op("A", 5));
        a.apply(a.decr_op("A"));
        a.apply(a.stepdown_op("A", 2));
        assert_eq!(a.counter(), 3);

        let mut b = PNCounter::new();
        b.apply(b.inc_op("B"));
        b.apply(b.stepup_op("B", 2));
        b.apply(b.inc_op("A"));
        assert_eq!(b.counter(), 4);

        a.merge(b);
        assert_eq!(a.counter(), 6);
    }

    #[test]
    fn test_inc_decr_step_up_down() {
        let mut a = PNCounter::new();
        a.inc("A");
        a.step_up("A", 5);
        a.decr("A");
        a.step_down("A", 2);

        assert_eq!(a.counter(), 3);
    }
}

use crate::{CmRDT, CvRDT, Dot, VClock};

use std::fmt::Debug;
use std::iter::FromIterator;
use std::ops::{AddAssign, BitOrAssign, SubAssign};

/// A pn-counter.
pub struct PnCounter<A: Ord> {
    increments: VClock<A>,
    decrements: VClock<A>,
}

/// The operations allowed by the PnCounter CDRT.
pub enum PnCounterOp<A> {
    Incr(Dot<A>),
    Decr(Dot<A>),
}

//
// Public api
//

impl<A: Ord> Default for PnCounter<A> {
    fn default() -> Self {
        Self {
            increments: Default::default(),
            decrements: Default::default(),
        }
    }
}

impl<A: Ord> PnCounter<A> {
    /// Creates a new instace of a gcounter.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<A: Clone + Ord> PnCounter<A> {
    /// Returns the value of the counter.
    pub fn value(&self) -> u64 {
        let i: u64 = self.increments.iter().map(|d| d.counter).sum();
        let d: u64 = self.decrements.iter().map(|d| d.counter).sum();
        i - d
    }
}

impl<A: Copy + Ord> PnCounter<A> {
    /// Increments the value for a given actor with a given step.
    pub fn step_up(&mut self, actor: &A, s: u64) {
        let dot = self.increments.dot(actor).step(s);
        self.increments.apply_op(dot);
    }

    /// Increments the value for a given actor.
    pub fn incr(&mut self, actor: &A) {
        self.step_up(actor, 1);
    }

    /// Decrements the value for a given actor with a given step.
    pub fn step_down(&mut self, actor: &A, s: u64) {
        let dot = self.decrements.dot(actor).step(s);
        self.decrements.apply_op(dot);
    }

    /// Decrements the value for a given actor.
    pub fn decr(&mut self, actor: &A) {
        self.step_down(actor, 1);
    }
}

//
// CRDT
//

impl<A: Ord> CmRDT for PnCounter<A> {
    type Op = PnCounterOp<A>;

    fn apply_op(&mut self, op: Self::Op) {
        match op {
            PnCounterOp::Incr(dot) => self.increments |= dot,
            PnCounterOp::Decr(dot) => self.decrements |= dot,
        }
    }
}

impl<A: Ord> CvRDT for PnCounter<A> {
    fn merge(&mut self, other: Self) {
        self.increments |= other.increments;
        self.decrements |= other.decrements;
    }
}

//
// Operations
//

impl<A: Ord> BitOrAssign for PnCounter<A> {
    fn bitor_assign(&mut self, rhs: Self) {
        self.merge(rhs);
    }
}

impl<A: Ord> AddAssign<Dot<A>> for PnCounter<A> {
    fn add_assign(&mut self, rhs: Dot<A>) {
        self.increments.apply_op(rhs);
    }
}

impl<A: Copy + Ord> AddAssign<A> for PnCounter<A> {
    fn add_assign(&mut self, rhs: A) {
        self.incr(&rhs);
    }
}

impl<A: Ord> SubAssign<Dot<A>> for PnCounter<A> {
    fn sub_assign(&mut self, rhs: Dot<A>) {
        self.decrements.apply_op(rhs);
    }
}

impl<A: Copy + Ord> SubAssign<A> for PnCounter<A> {
    fn sub_assign(&mut self, rhs: A) {
        self.decr(&rhs);
    }
}

//
// Froms
//

impl<A: Ord> FromIterator<A> for PnCounter<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let xs = iter
            .into_iter()
            .map(|a| (a, 0u64))
            .collect::<Vec<(A, u64)>>();

        Self {
            increments: VClock::from_iter(xs),
            decrements: Default::default(),
        }
    }
}

impl<A: Ord> FromIterator<(A, u64)> for PnCounter<A> {
    fn from_iter<T: IntoIterator<Item = (A, u64)>>(iter: T) -> Self {
        Self {
            increments: VClock::from_iter(iter),
            decrements: Default::default(),
        }
    }
}

//
// Formatting
//

impl<A: Debug + Ord> Debug for PnCounter<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self.increments)?;
        writeln!(f, "{:?}", self.decrements)
    }
}

//
// Tests
//

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[test]
    fn test_new() {
        type A = i32;
        let pnc = PnCounter::<A>::new();
        assert!(pnc.increments.is_empty());
        assert!(pnc.decrements.is_empty());
    }

    #[quickcheck]
    fn test_from_iter(len: usize) -> bool {
        let len = len % 100;
        let mut actors = Vec::with_capacity(len);
        for i in 0..len {
            actors.push(i as i32);
        }

        let pnc = PnCounter::<i32>::from_iter(actors);
        assert!(pnc.decrements.is_empty());
        pnc.increments.len() == len
    }

    #[quickcheck]
    fn test_from_pairs(len: usize) -> bool {
        let len = len % 100;
        let mut pairs = Vec::with_capacity(len);
        let mut ttl = 0;
        for i in 0..len {
            ttl += (i + 10) as u64;
            pairs.push((i as i32, (i + 10) as u64));
        }

        let pnc = PnCounter::<i32>::from_iter(pairs);
        assert_eq!(ttl, pnc.value());
        assert!(pnc.decrements.is_empty());
        pnc.increments.len() == len
    }

    #[test]
    fn test_debug() {
        let gc = PnCounter::<i32>::from_iter([1, 2, 3]);
        let s = format!("{:?}", gc);
        assert!(!s.is_empty())
    }

    #[test]
    fn test_add_assign_dot() {
        let mut pnc = PnCounter::<i32>::new();

        pnc += Dot::new(1, 10);
        assert_eq!(pnc.increments.len(), 1);
        assert_eq!(pnc.value(), 10);

        pnc += Dot::new(2, 20);
        assert_eq!(pnc.increments.len(), 2);
        assert_eq!(pnc.value(), 30);
    }

    #[test]
    fn test_add_assign_actor() {
        let mut pnc = PnCounter::<i32>::new();

        pnc += 1; // the actor id
        assert_eq!(pnc.increments.len(), 1);
        assert_eq!(pnc.value(), 1);

        pnc += 2; // the actor id
        assert_eq!(pnc.increments.len(), 2);
        assert_eq!(pnc.value(), 2);
    }

    #[test]
    fn test_sub_assign_dot() {
        let mut pnc = PnCounter::<i32>::from_iter([(1, 10), (2, 0), (3, 20)]);

        pnc -= Dot::new(1, 2);
        assert_eq!(pnc.value(), 10 + 20 - 2);
    }

    #[test]
    fn test_sub_assign_actor() {
        let mut pnc = PnCounter::<i32>::from_iter([(1, 10), (2, 0), (3, 20)]);

        pnc -= 1;
        assert_eq!(pnc.value(), 10 + 20 - 1);
    }

    #[test]
    fn test_bitor_assign_gcounter() {
        let mut a = PnCounter::<i32>::from_iter([(1, 10), (2, 0), (3, 20)]);
        let b = PnCounter::<i32>::from_iter([(1, 5), (2, 5)]);

        a |= b;

        assert_eq!(a.increments.len(), 3);
        assert_eq!(a.value(), 10 + 5 + 20);
    }

    #[test]
    fn test_crdt_apply_op() {
        let mut pnc = PnCounter::<i32>::new();

        pnc.apply_op(PnCounterOp::Incr(Dot::new(1, 20)));
        assert_eq!(pnc.increments.len(), 1);
        assert_eq!(pnc.value(), 20);

        pnc.apply_op(PnCounterOp::Decr(Dot::new(2, 10)));
        assert_eq!(pnc.decrements.len(), 1);

        assert_eq!(pnc.value(), 20 - 10);
    }

    #[test]
    fn test_crdt_merge() {
        let mut a = PnCounter::<i32>::from_iter([(1, 10), (2, 0), (3, 20)]);
        let b = PnCounter::<i32>::from_iter([(1, 5), (2, 5)]);

        a.merge(b);

        assert_eq!(a.increments.len(), 3);
        assert_eq!(a.value(), 10 + 5 + 20);
    }

    #[test]
    fn test_step_up() {
        let mut a = PnCounter::<i32>::from_iter([(1, 10), (2, 0), (3, 20)]);
        a.step_up(&1, 5);
        assert_eq!(10 + 5 + 20, a.value());
    }

    #[test]
    fn test_incr() {
        let mut a = PnCounter::<i32>::from_iter([(1, 10), (2, 0), (3, 20)]);
        a.incr(&1);
        assert_eq!(10 + 1 + 20, a.value());
    }

    #[test]
    fn test_step_down() {
        let mut a = PnCounter::<i32>::from_iter([(1, 10), (2, 0), (3, 20)]);
        a.step_down(&1, 5);
        assert_eq!(10 - 5 + 20, a.value());
    }

    #[test]
    fn test_decr() {
        let mut a = PnCounter::<i32>::from_iter([(1, 10), (2, 0), (3, 20)]);
        a.decr(&1);
        assert_eq!(10 - 1 + 20, a.value());
    }
}

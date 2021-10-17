use crate::{CmRDT, CvRDT, Dot, VClock};

use std::fmt::Debug;
use std::iter::FromIterator;
use std::ops::BitOrAssign;

/// A grow-counter CDRT
///
/// # Example
///
/// ```rust
/// use euklid_clocks::*;
/// use std::iter::FromIterator;
///
/// let mut a = GCounter::<i32>::from_iter([(1, 10), (2, 0), (3, 20)]);
/// let b = GCounter::<i32>::from_iter([(1, 5), (2, 5)]);
///
/// // Merging the two counters.
/// a |= b;
///
/// assert_eq!(a.value(), 10 + 5 + 20);
/// ```
pub struct GCounter<A: Ord> {
    counters: VClock<A>,
}

//
// Public api
//

impl<A: Ord> Default for GCounter<A> {
    fn default() -> Self {
        Self {
            counters: Default::default(),
        }
    }
}

impl<A: Ord> GCounter<A> {
    /// Creates a new instace of a gcounter.
    pub fn new() -> Self {
        Self::default()
    }

    /// Merges the current counters with a given dot value.
    pub fn merge_dot(&mut self, dot: Dot<A>) {
        self.apply_op(dot);
    }
}

impl<A: Clone + Ord> GCounter<A> {
    /// Returns the value of the counter.
    pub fn value(&self) -> u64 {
        self.counters.iter().map(|d| d.counter).sum()
    }
}

//
// CRDT
//

impl<A: Ord> CmRDT for GCounter<A> {
    type Op = Dot<A>;

    fn apply_op(&mut self, op: Self::Op) {
        self.counters |= op;
    }
}

impl<A: Ord> CvRDT for GCounter<A> {
    fn merge(&mut self, other: Self) {
        self.counters |= other.counters;
    }
}

//
// Operations
//

impl<A: Ord> BitOrAssign<Dot<A>> for GCounter<A> {
    fn bitor_assign(&mut self, rhs: Dot<A>) {
        self.merge_dot(rhs);
    }
}

impl<A: Ord> BitOrAssign for GCounter<A> {
    fn bitor_assign(&mut self, rhs: Self) {
        self.merge(rhs);
    }
}

//
// Froms
//

impl<A: Ord> FromIterator<A> for GCounter<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let xs = iter
            .into_iter()
            .map(|a| (a, 0u64))
            .collect::<Vec<(A, u64)>>();

        Self {
            counters: VClock::from_iter(xs),
        }
    }
}

impl<A: Ord> FromIterator<(A, u64)> for GCounter<A> {
    fn from_iter<T: IntoIterator<Item = (A, u64)>>(iter: T) -> Self {
        Self {
            counters: VClock::from_iter(iter),
        }
    }
}

//
// Formatting
//

impl<A: Debug + Ord> Debug for GCounter<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.counters)
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
        let gc = GCounter::<A>::new();
        assert!(gc.counters.is_empty());
        assert_eq!(gc.counters.len(), 0);
    }

    #[quickcheck]
    fn test_from_iter(len: usize) -> bool {
        let len = len % 100;
        let mut actors = Vec::with_capacity(len);
        for i in 0..len {
            actors.push(i as i32);
        }

        let gc = GCounter::<i32>::from_iter(actors);
        gc.counters.len() == len
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

        let gc = GCounter::<i32>::from_iter(pairs);
        assert_eq!(ttl, gc.value());
        gc.counters.len() == len
    }

    #[test]
    fn test_debug() {
        let gc = GCounter::<i32>::from_iter([1, 2, 3]);
        let s = format!("{:?}", gc);
        assert!(!s.is_empty())
    }

    #[test]
    fn test_bitor_assign_dot() {
        let mut gc = GCounter::<i32>::new();

        gc |= Dot::new(1, 10);
        assert_eq!(gc.counters.len(), 1);
        assert_eq!(gc.value(), 10);

        gc |= Dot::new(2, 20);
        assert_eq!(gc.counters.len(), 2);
        assert_eq!(gc.value(), 30);
    }

    #[test]
    fn test_bitor_assign_gcounter() {
        let mut a = GCounter::<i32>::from_iter([(1, 10), (2, 0), (3, 20)]);
        let b = GCounter::<i32>::from_iter([(1, 5), (2, 5)]);

        a |= b;

        assert_eq!(a.counters.len(), 3);
        assert_eq!(a.value(), 10 + 5 + 20);
    }

    #[test]
    fn test_crdt_apply_op() {
        let mut gc = GCounter::<i32>::new();

        gc.apply_op(Dot::new(1, 10));
        assert_eq!(gc.counters.len(), 1);
        assert_eq!(gc.value(), 10);

        gc.apply_op(Dot::new(2, 20));
        assert_eq!(gc.counters.len(), 2);
        assert_eq!(gc.value(), 30);
    }

    #[test]
    fn test_crdt_merge() {
        let mut a = GCounter::<i32>::from_iter([(1, 10), (2, 0), (3, 20)]);
        let b = GCounter::<i32>::from_iter([(1, 5), (2, 5)]);

        a.merge(b);

        assert_eq!(a.counters.len(), 3);
        assert_eq!(a.value(), 10 + 5 + 20);
    }

    #[test]
    fn test_merge_dot() {
        let mut gc = GCounter::<i32>::new();

        gc.merge_dot(Dot::new(1, 10));
        assert_eq!(gc.counters.len(), 1);
        assert_eq!(gc.value(), 10);

        gc.merge_dot(Dot::new(2, 20));
        assert_eq!(gc.counters.len(), 2);
        assert_eq!(gc.value(), 30);
    }
}

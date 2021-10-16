use crate::CausalOrd;

use super::{CmRDT, CvRDT, Dot};
use std::{
    cmp::Ordering,
    collections::{btree_map, BTreeMap},
    fmt::Debug,
    iter::FromIterator,
};

/// A vector clock.
pub struct VClock<A: Ord> {
    /// dots store the map between actors and their associated counters
    dots: BTreeMap<A, u64>,
}

impl<A: Ord> Default for VClock<A> {
    fn default() -> Self {
        Self {
            dots: Default::default(),
        }
    }
}

impl<A: Ord + Debug> Debug for VClock<A> {
    /// Formats the display string
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let xs = self
            .dots
            .iter()
            .map(|(a, c)| format!("{:?}:{}", a, c))
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "<{}>", xs)
    }
}

//
// Causal order
//

impl<A: Ord> PartialEq for VClock<A> {
    fn eq(&self, other: &Self) -> bool {
        self.dots == other.dots
    }
}

impl<A: Ord> PartialOrd for VClock<A> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if other.dots.iter().all(|(a, c)| self.counter(a) >= *c) {
            Some(Ordering::Greater)
        } else if other.dots.iter().all(|(a, c)| self.counter(a) <= *c) {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}

impl<A: Ord> CausalOrd for VClock<A> {}

impl<A: Ord> VClock<A> {
    /// Creates a new instance of a ['VClock']
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns true if there are no dots in the vector
    pub fn is_empty(&self) -> bool {
        self.dots.is_empty()
    }

    /// Returns the number of dots stored in the vector
    pub fn len(&self) -> usize {
        self.dots.len()
    }

    /// Returns the counter for a given actor. If the actor
    /// is not stored in the vector, it returns 0.
    pub(crate) fn counter(&self, actor: &A) -> u64 {
        self.dots.get(actor).cloned().unwrap_or(0)
    }
}

impl<A: Ord + Copy> VClock<A> {
    /// Returns the dot for the given actor.
    pub fn dot(&self, actor: &A) -> Dot<A> {
        let counter = self.counter(actor);
        Dot::new(*actor, counter)
    }

    /// Returns an iterator over the dots in this vclock
    pub fn iter(&self) -> impl Iterator<Item = Dot<A>> + '_ {
        self.dots.iter().map(|(a, c)| Dot {
            actor: *a,
            counter: *c,
        })
    }
}

//
// CRDT
//

impl<A: Ord> CvRDT for VClock<A> {
    fn merge(&mut self, other: Self) {
        // Merge each other dot into the current vector clock
        for dot in other.into_iter() {
            self.apply_op(dot);
        }
    }
}

impl<A: Ord> CmRDT for VClock<A> {
    type Op = Dot<A>;

    fn apply_op(&mut self, op: Self::Op) {
        // We apply the operation if the new counter is greater than then existing one.
        if self.counter(&op.actor) < op.counter {
            self.dots.insert(op.actor, op.counter);
        }
    }
}

//
// Iterators
//

/// Helper structure to generate an iterator based on the content of the vector clock.
pub struct IntoIter<A: Ord> {
    btree_iter: btree_map::IntoIter<A, u64>,
}

impl<A: Ord> std::iter::Iterator for IntoIter<A> {
    type Item = Dot<A>;

    fn next(&mut self) -> Option<Dot<A>> {
        self.btree_iter
            .next()
            .map(|(actor, counter)| Dot::new(actor, counter))
    }
}

impl<A: Ord> std::iter::IntoIterator for VClock<A> {
    type Item = Dot<A>;
    type IntoIter = IntoIter<A>;

    /// Consumes the vclock and returns an iterator over dots in the clock
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            btree_iter: self.dots.into_iter(),
        }
    }
}

impl<A: Ord> FromIterator<A> for VClock<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let xs = iter
            .into_iter()
            .map(|a| (a, 0u64))
            .collect::<Vec<(A, u64)>>();
        Self {
            dots: BTreeMap::<A, u64>::from_iter(xs),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[test]
    fn test_new() {
        let vc = VClock::<i32>::default();
        assert!(vc.is_empty());
        assert_eq!(0, vc.len());
    }

    #[quickcheck]
    fn test_from_iter(len: usize) -> bool {
        let len = len % 100;
        let mut actors = Vec::new();
        for i in 0..len {
            actors.push(i as i32);
        }

        let vc = VClock::<i32>::from_iter(actors);
        vc.len() == len
    }

    #[test]
    fn test_debug() {
        let vc = VClock::<i32>::from_iter([1, 2, 3]);
        let s = format!("{:?}", vc);
        assert!(!s.is_empty());
    }

    #[test]
    fn test_eq() {
        let xs = VClock::<i32>::from_iter([1, 2, 3]);
        let ys = VClock::<i32>::from_iter([1, 2, 3]);
        assert!(xs.eq(&ys));
    }

    #[test]
    fn test_crdt_apply_dot() {
        let mut v = VClock::<i32>::from_iter([1, 2, 3]);
        let a = (1, 10).into();
        v.apply_op(a);

        assert_eq!(10, v.counter(&1));
        assert_eq!(0, v.counter(&2));
        assert_eq!(0, v.counter(&3));
    }

    #[test]
    fn test_crdt_merge() {
        let mut v1 = VClock::<i32>::from_iter([1, 2, 3]);
        v1.apply_op((1, 10).into());
        v1.apply_op((3, 30).into());

        let mut v2 = VClock::<i32>::from_iter([1, 2, 3, 4]);
        v2.apply_op((1, 15).into());
        v2.apply_op((2, 20).into());
        v2.apply_op((3, 28).into());

        v1.merge(v2);

        assert_eq!(15, v1.counter(&1));
        assert_eq!(20, v1.counter(&2));
        assert_eq!(30, v1.counter(&3));
        assert_eq!(0, v1.counter(&4));
    }

    #[test]
    fn test_pord_eq() {
        let xs = VClock::<i32>::from_iter([1, 2, 3]);
        let ys = VClock::<i32>::from_iter([1, 2, 3]);
        assert_eq!(Some(Ordering::Equal), xs.partial_cmp(&ys));
    }

    #[test]
    fn test_pord_gr() {
        let mut v1 = VClock::<i32>::from_iter([1, 2, 3]);
        v1.apply_op((1, 10).into());

        let v2 = VClock::<i32>::from_iter([1, 2, 3]);

        assert!(matches!(v1.partial_cmp(&v2), Some(Ordering::Greater)));
    }

    #[test]
    fn test_pord_ls() {
        let mut v1 = VClock::<i32>::from_iter([1, 2, 3]);
        v1.apply_op((1, 10).into());

        let v2 = VClock::<i32>::from_iter([1, 2, 3]);

        assert!(matches!(v2.partial_cmp(&v1), Some(Ordering::Less)));
    }

    #[test]
    fn test_default() {
        let v = VClock::<i32>::default();
        assert!(v.is_empty());
        assert_eq!(0, v.len());
    }

    #[test]
    fn test_dot() {
        let xs = VClock::<i32>::from_iter([1, 2, 3]);
        let d = xs.dot(&1);
        assert_eq!(1, d.actor);
        assert_eq!(0, d.counter);
    }

    #[test]
    fn test_iter() {
        let xs = VClock::<i32>::from_iter([1, 2, 3]);
        assert_eq!(3, xs.iter().count());
    }
}

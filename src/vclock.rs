use std::collections::{btree_map, BTreeMap};
use std::convert::Infallible;
use std::fmt::{Debug, Display};
use std::ops::AddAssign;
use itertools::Itertools;

use crate::{Actor, CausalityOrd, CmRDT, Counter, CvRDT, Dot, DotRange, UpdateAssign};

/// A vclock is a collection of dots, one for each actor.
pub struct VClock<A: Actor, C: Counter> {
    dots: BTreeMap<A, C>,
}

//
// Constructors
//

impl<A: Actor, C: Counter> Default for VClock<A, C> {
    fn default() -> Self {
        Self {
            dots: BTreeMap::new(),
        }
    }
}

impl<A: Actor, C: Counter> From<(A, C)> for VClock<A, C> {
    fn from(pair: (A, C)) -> Self {
        let mut clock = VClock::default();
        clock.dots.insert(pair.0, pair.1);
        clock
    }
}

impl<A: Actor, C: Counter> From<Dot<A, C>> for VClock<A, C> {
    fn from(dot: Dot<A, C>) -> Self {
        let mut clock = VClock::default();
        clock.dots.insert(dot.actor, dot.counter);
        clock
    }
}

impl<A: Actor, C: Counter> From<Vec<(A, C)>> for VClock<A, C> {
    fn from(pairs: Vec<(A, C)>) -> Self {
        let mut clock = Self::default();
        let _: Vec<&(A, C)> = pairs
            .iter()
            .inspect(|(a, c)| {
                clock.dots.insert(*a, *c);
                ()
            })
            .collect();
        clock
    }
}

//
// Formatting traits
//

impl<A: Actor + Display, C: Counter + Display> Display for VClock<A, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let dots = self.dots.iter().map(|(a,c)| format!("{}:{}", *a, *c)).join(",");

        write!(f, "<")?;
        write!(f, "{}", dots)?;
        write!(f, ">")
    }
}

//
// Order traits
//

impl<A: Actor, C: Counter> PartialEq for VClock<A, C> {
    fn eq(&self, other: &Self) -> bool {
        if self.dots.len() != other.dots.len() {
            false
        } else {
            self.dots
                .iter()
                .all(|(a, c)| other.dots.get(&a).map_or(false, |d| d == c))
        }
    }
}

impl<A: Actor, C: Counter> PartialOrd for VClock<A, C> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            Some(core::cmp::Ordering::Equal)
        } else if self.dots.iter().all(|(a, c)| other.contains_greater(a, c)) {
            Some(core::cmp::Ordering::Less)
        } else if self.dots.iter().all(|(a, c)| other.contains_less(a, c)) {
            Some(core::cmp::Ordering::Greater)
        } else {
            None
        }
    }
}

impl<A: Actor, C: Counter> CausalityOrd for VClock<A, C> {}

//
// Operations
//

impl<A: Actor, C: Counter> AddAssign<Dot<A, C>> for VClock<A, C> {
    fn add_assign(&mut self, rhs: Dot<A, C>) {
        let counter = self.get_counter_or_zero(&rhs.actor) + rhs.counter;
        self.dots.insert(rhs.actor, counter);
    }
}

impl<A: Actor, C: Counter> UpdateAssign<Dot<A, C>> for VClock<A, C> {
    fn upd_assign(&mut self, rhs: Dot<A, C>) {
        self.dots.insert(rhs.actor, rhs.counter);
    }
}

//
// Implemenation
//

impl<A: Actor, C: Counter> VClock<A, C> {
    /// Constructs a new `VClock` instance.
    pub fn new(actor: &A, count: &C) -> Self {
        let mut clock = Self {
            dots: BTreeMap::new(),
        };

        clock.dots.insert(*actor, *count);
        clock
    }

    /// Returns `true` is the clock contains dots.
    pub fn is_empty(&self) -> bool {
        self.dots.is_empty()
    }

    /// Returns the number of dots in the clock.
    pub fn len(&self) -> usize {
        self.dots.len()
    }

    /// Returns the `dot` stored for a given actor
    pub fn dot_or_zero(&self, actor: &A) -> Dot<A, C> {
        self.dots
            .get_key_value(actor)
            .map_or(Dot::new_zero(*actor), |(a, c)| Dot::new(*a, *c))
    }

    /// Returns a dot with an incremented for a given
    pub fn dot_incremented(&self, actor: &A) -> Dot<A, C> {
        self.dot_or_zero(actor).incr()
    }

    /// Determines if stored counter for a given actor is greater than the specified counter.
    pub(crate) fn contains_greater(&self, actor: &A, count: &C) -> bool {
        self.dots.get(actor).map_or(false, |c| c >= count)
    }

    /// Determines if stored counter for a given actor is smaller than the specified counter.
    pub(crate) fn contains_less(&self, actor: &A, count: &C) -> bool {
        self.dots.get(actor).map_or(false, |c| c <= count)
    }

    /// Returns an iterator over the dots in this vclock
    pub fn iter(&self) -> impl Iterator<Item = Dot<A, C>> + '_ {
        self.dots.iter().map(|(a, c)| Dot::new(*a, *c))
    }

    fn get_counter_or(&self, default: C, actor: &A) -> C {
        *self.dots.get(actor).unwrap_or(&default)
    }

    fn get_counter_or_zero(&self, actor: &A) -> C {
        self.get_counter_or(C::zero(), actor)
    }

    fn apply_dot(&mut self, dot: Dot<A, C>) {
        if self.get_counter_or_zero(&dot.actor) < dot.counter {
            self.dots.insert(dot.actor, dot.counter);
        }
    }

    /// Increments in-place the `dot` for a specified actor.
    pub(crate) fn incr_assign(&mut self, actor: &A) {
        *self += Dot::new_one(*actor);
    }
}

//
// Iterator
//

/// Simple iterator state
pub struct IntoIter<A: Actor, C: Counter> {
    /// The iter
    btree_iter: btree_map::IntoIter<A, C>,
}

impl<A: Actor, C: Counter> std::iter::Iterator for IntoIter<A, C> {
    type Item = Dot<A, C>;

    fn next(&mut self) -> Option<Self::Item> {
        self.btree_iter
            .next()
            .map(|(actor, count)| Dot::new(actor, count))
    }
}

impl<A: Actor, C: Counter> std::iter::IntoIterator for VClock<A, C> {
    type Item = Dot<A, C>;
    type IntoIter = IntoIter<A, C>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            btree_iter: self.dots.into_iter(),
        }
    }
}

//
// CvRDT and CmRDT
//

impl<A: Actor, C: Counter> CvRDT for VClock<A, C> {
    type Validation = Infallible;

    fn validate_merge(&self, _: &Self) -> Result<(), Self::Validation> {
        Ok(())
    }

    fn merge(&mut self, other: Self) {
        for dot in other.into_iter() {
            self.apply_dot(dot);
        }
    }
}

impl<A: Actor + Debug + Display, C: Counter + Debug + Display> CmRDT for VClock<A, C> {
    type Op = Dot<A, C>;

    type Validation = DotRange<A, C>;

    fn validate_op(&self, dot: &Self::Op) -> Result<(), Self::Validation> {
        let next_counter = self.get_counter_or_zero(&dot.actor).incr();
        if dot.counter > next_counter {
            Err(DotRange {
                actor: dot.actor.clone(),
                range: next_counter..dot.counter,
            })
        } else {
            Ok(())
        }
    }

    fn apply(&mut self, dot: Self::Op) {
        if self.get_counter_or_zero(&dot.actor) < dot.counter {
            self.dots.insert(dot.actor, dot.counter);
        }
    }
}

//
// Tests
//

#[cfg(test)]
mod tests {
    use crate::Causality;

    use super::*;
    use quickcheck_macros::quickcheck;

    #[test]
    fn test_default() {
        let clock: VClock<i8, usize> = VClock::default();
        assert!(clock.is_empty());
    }

    #[quickcheck]
    fn test_new(actor: i8, count: usize) -> bool {
        let clock = VClock::new(&actor, &count);
        !clock.is_empty()
    }

    #[quickcheck]
    fn test_from_pair(actor: i8, count: usize) -> bool {
        let clock: VClock<i8, usize> = (actor, count).into();
        !clock.is_empty() && clock.len() == 1
    }

    #[test]
    fn test_from_vec() {
        let dots: Vec<(i8, usize)> = vec![(1, 10), (2, 20), (3, 30)];
        let clock: VClock<i8, usize> = dots.into();
        assert_eq!(3, clock.len())
    }

    #[quickcheck]
    fn test_from_dot(actor: i8, count: usize) -> bool {
        let dot: Dot<i8, usize> = (actor, count).into();
        let clock: VClock<i8, usize> = dot.into();
        !clock.is_empty() && clock.len() == 1
    }

    #[test]
    fn test_dot_or_zero() {
        let dots: Vec<(i8, usize)> = vec![(1, 10), (2, 20), (3, 30)];
        let clock: VClock<i8, usize> = dots.into();

        let dot = clock.dot_or_zero(&2);
        assert!(dot.actor == 2 && dot.counter == 20);

        let dot = clock.dot_or_zero(&4);
        assert!(dot.actor == 4 && dot.counter == 0);
    }

    #[test]
    fn test_dot_incremented() {
        let dots: Vec<(i8, usize)> = vec![(1, 10), (2, 20), (3, 30)];
        let clock: VClock<i8, usize> = dots.into();

        let dot = clock.dot_incremented(&2);
        assert!(dot.actor == 2 && dot.counter == 21);

        let dot = clock.dot_incremented(&4);
        assert!(dot.actor == 4 && dot.counter == 1);
    }

    #[test]
    fn test_contains_greater() {
        let clock: VClock<i8, usize> = vec![(1, 10), (2, 20), (3, 30)].into();
        assert!(clock.contains_greater(&2, &10));
        assert!(!clock.contains_greater(&2, &30));
        assert!(!clock.contains_greater(&4, &10));
    }

    #[test]
    fn test_contains_less() {
        let clock: VClock<i8, usize> = vec![(1, 10), (2, 20), (3, 30)].into();
        assert!(!clock.contains_less(&2, &10));
        assert!(clock.contains_less(&2, &30));
        assert!(!clock.contains_less(&4, &10));
    }

    #[test]
    fn test_causality_eq() {
        let clock1: VClock<i8, usize> = vec![(1, 10), (2, 20), (3, 30)].into();
        let clock2: VClock<i8, usize> = vec![(1, 10), (2, 20), (3, 30)].into();

        assert_eq!(clock1.causality_cmp(&clock2), Causality::Equal);
    }

    #[test]
    fn test_causality_precede() {
        let clock1: VClock<i8, usize> = vec![(1, 10), (2, 10), (3, 30)].into();
        let clock2: VClock<i8, usize> = vec![(1, 10), (2, 20), (3, 30)].into();

        assert_eq!(clock1.causality_cmp(&clock2), Causality::Precede);
    }

    #[test]
    fn test_causality_succeed() {
        let clock1: VClock<i8, usize> = vec![(1, 10), (2, 30), (3, 30)].into();
        let clock2: VClock<i8, usize> = vec![(1, 10), (2, 20), (3, 30)].into();

        assert_eq!(clock1.causality_cmp(&clock2), Causality::Succeed);
    }

    #[test]
    fn test_causality_concurrent() {
        let clock1: VClock<i8, usize> = vec![(1, 10), (2, 10), (3, 30)].into();
        let clock2: VClock<i8, usize> = vec![(1, 10), (2, 20), (3, 20)].into();

        assert_eq!(clock1.causality_cmp(&clock2), Causality::Concurrent);
    }

    #[test]
    fn test_causality_precede_diff_len() {
        let clock1: VClock<i8, usize> = vec![(1, 10), (2, 20), (3, 30)].into();
        let clock2: VClock<i8, usize> = vec![(1, 10), (2, 20), (3, 30), (4, 40)].into();

        assert_eq!(clock1.causality_cmp(&clock2), Causality::Precede);
    }

    #[test]
    fn test_causality_concurrent_diff_len() {
        let clock1: VClock<i8, usize> = vec![(1, 10), (2, 20), (3, 30)].into();
        let clock2: VClock<i8, usize> = vec![(1, 10), (2, 20), (3, 30), (4, 40)].into();

        assert_eq!(clock2.causality_cmp(&clock1), Causality::Concurrent);
    }

    #[test]
    fn test_eq_diff_len() {
        let clock1: VClock<i8, usize> = vec![(1, 10), (2, 20), (3, 30)].into();
        let clock2: VClock<i8, usize> = vec![(1, 10), (2, 20), (3, 30), (4, 40)].into();

        assert!(clock1 != clock2);
        assert!(clock2 != clock1);
    }

    #[test]
    fn test_eq_diff_item() {
        let clock1: VClock<i8, usize> = vec![(1, 10), (2, 20), (3, 30)].into();
        let clock2: VClock<i8, usize> = vec![(1, 10), (2, 20), (4, 40)].into();

        assert!(clock1 != clock2);
        assert!(clock2 != clock1);
    }

    #[test]
    fn test_iter() {
        let clock: VClock<i8, i32> = vec![(1, 10), (2, 20), (3, 30)].into();
        let ttl = clock.iter().map(|dot| dot.counter).sum();
        assert_eq!(60, ttl);
    }

    #[test]
    fn test_intoiter() {
        let clock: VClock<i8, i32> = vec![(1, 10), (2, 20), (3, 30)].into();
        let mut iter = clock.into_iter();
        assert_eq!(iter.next().map(|d| d.counter).unwrap(), 10);
        assert_eq!(iter.next().map(|d| d.counter).unwrap(), 20);
        assert_eq!(iter.next().map(|d| d.counter).unwrap(), 30);
    }

    #[test]
    fn merge() {
        let mut clock1: VClock<i8, usize> = vec![(1, 10), (2, 20), (3, 30)].into();
        let clock2: VClock<i8, usize> = vec![(1, 100), (2, 10), (4, 40)].into();

        clock1.merge(clock2);
        assert_eq!(100, clock1.get_counter_or_zero(&1));
        assert_eq!(20, clock1.get_counter_or_zero(&2));
        assert_eq!(30, clock1.get_counter_or_zero(&3));
        assert_eq!(40, clock1.get_counter_or_zero(&4));
    }

    #[test]
    fn apply_dot() {
        let mut clock: VClock<i8, usize> = vec![(1, 10), (2, 20), (3, 30)].into();

        let dot1: Dot<i8, usize> = Dot::new(1, 100);
        clock.apply(dot1);
        assert_eq!(clock.get_counter_or_zero(&1), 100);

        let dot2: Dot<i8, usize> = Dot::new(2, 10);
        clock.apply(dot2);
        assert_eq!(clock.get_counter_or_zero(&2), 20);

        let dot3: Dot<i8, usize> = Dot::new(4, 40);
        clock.apply(dot3);
        assert_eq!(clock.get_counter_or_zero(&4), 40);
    }

    #[test]
    fn incr_assign_existing() {
        let mut clock: VClock<i8, usize> = vec![(1, 10), (2, 20), (3, 30)].into();
        clock.incr_assign(&1);
        assert_eq!(11, clock.get_counter_or_zero(&1));
    }

    #[test]
    fn incr_assign_notexisting() {
        let mut clock: VClock<i8, usize> = vec![(1, 10), (2, 20), (3, 30)].into();
        clock.incr_assign(&4);
        assert_eq!(1, clock.get_counter_or_zero(&4));
    }

    #[test]
    fn add_assign_existing() {
        let mut clock: VClock<i8, usize> = vec![(1, 10), (2, 20), (3, 30)].into();
        clock += Dot::new_one(1);
        assert_eq!(11, clock.get_counter_or_zero(&1));
    }

    #[test]
    fn add_assign_notexisting() {
        let mut clock: VClock<i8, usize> = vec![(1, 10), (2, 20), (3, 30)].into();
        clock += Dot::new_one(4);
        assert_eq!(1, clock.get_counter_or_zero(&4));
    }

    #[test]
    fn upd_assign_existing() {
        let mut clock: VClock<i8, usize> = vec![(1, 10), (2, 20), (3, 30)].into();
        clock.upd_assign(Dot::new_one(1));
        assert_eq!(1, clock.get_counter_or_zero(&1));
    }

    #[test]
    fn upd_assign_notexisting() {
        let mut clock: VClock<i8, usize> = vec![(1, 10), (2, 20), (3, 30)].into();
        clock.upd_assign(Dot::new_one(4));
        assert_eq!(1, clock.get_counter_or_zero(&4));
    }
}

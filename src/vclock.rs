use core::cmp::Ordering;
use core::fmt::{Debug, Display};
use std::collections::{btree_map, BTreeMap};

use crate::{CmRDT, CvRDT, Dot};

/// A vector clock
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VClock<A: Ord> {
    /// dots store the map between actors and their associated counters
    dots: BTreeMap<A, u64>,
}

impl<A: Ord> Default for VClock<A> {
    /// Builds an empty vector clock
    fn default() -> Self {
        Self {
            dots: Default::default(),
        }
    }
}

impl<A: Ord + Display> Display for VClock<A> {
    /// Formats the display string
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<")?;
        for (i, (actor, count)) in self.dots.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}:{}", actor, count)?;
        }
        write!(f, ">")
    }
}

//
// CRDT
//

impl<A: Ord + Clone + Debug> CmRDT for VClock<A> {
    type Op = Dot<A>;

    fn apply(&mut self, op: Self::Op) {
        // We apply the operation if the new counter is greater than then existing one.
        if self.counter_of(&op.actor) < op.counter {
            self.dots.insert(op.actor, op.counter);
        }
    }
}

impl<A: Ord + Clone + Debug> CvRDT for VClock<A> {
    fn merge(&mut self, other: Self) {
        // Merge each other dot into the current vector clock
        for dot in other.into_iter() {
            self.apply(dot);
        }
    }
}

//
// API
//

impl<A: Ord> VClock<A> {
    /// Build a new vector clock
    pub fn new() -> Self {
        Default::default()
    }

    /// Return 'true' if there are no dots
    pub fn is_empty(&self) -> bool {
        self.dots.is_empty()
    }

    /// Returns the counter for a given actor.
    /// If the actor is not in the vector clock, it returns 0.
    pub fn counter_of(&self, actor: &A) -> u64 {
        self.dots.get(actor).cloned().unwrap_or(0)
    }

    /// Reurns the dot for a given actor.
    pub fn dot_of(&self, actor: A) -> Dot<A> {
        let counter = self.counter_of(&actor);
        Dot::new(actor, counter)
    }

    /// Generate the Op to increment the counter for a given actor.
    ///
    /// # Examples
    /// ```
    /// use euklid::{VClock, CmRDT};
    /// let mut a = VClock::new();
    ///
    /// // `a.inc_op()` does not mutate the vclock!
    /// let op = a.inc_op("A");
    /// assert_eq!(a, VClock::new());
    ///
    /// // we must apply the op to the VClock to have
    /// // its edit take effect.
    /// a.apply(op.clone());
    /// assert_eq!(a.counter_of(&"A"), 1);
    /// ```
    pub fn inc_op(&self, actor: A) -> Dot<A>
    where
        A: Clone,
    {
        self.dot_of(actor).inc()
    }

    /// Generate the Op to increase the counter for a given actor with a given value.
    ///
    /// # Examples
    /// ```
    /// use euklid::{VClock, CmRDT};
    /// let mut a = VClock::new();
    ///
    /// // `a.step_op()` does not mutate the vclock!
    /// let op = a.step_op("A", 10);
    /// assert_eq!(a, VClock::new());
    ///
    /// // we must apply the op to the VClock to have
    /// // its edit take effect.
    /// a.apply(op.clone());
    /// assert_eq!(a.counter_of(&"A"), 10);
    /// ```
    pub fn step_op(&self, actor: A, s: u64) -> Dot<A>
    where
        A: Clone,
    {
        self.dot_of(actor).step(s)
    }

    /// Returns an iterator over the dots in this vclock
    pub fn iter(&self) -> impl Iterator<Item = Dot<&A>> {
        self.dots.iter().map(|(a, c)| Dot {
            actor: a,
            counter: *c,
        })
    }
}

//
// Iterator
//

/// Generated from calls to VClock::into_iter()
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

impl<A: Ord + Clone + Debug> From<Dot<A>> for VClock<A> {
    /// Builds a vector clock out of a dot.
    fn from(dot: Dot<A>) -> Self {
        let mut vclock = VClock::default();
        vclock.apply(dot);
        vclock
    }
}

impl<A: Ord + Clone + Debug> std::iter::FromIterator<Dot<A>> for VClock<A> {
    /// Builds a vector clock out of a list of dots.
    fn from_iter<I: IntoIterator<Item = Dot<A>>>(iter: I) -> Self {
        let mut vclock = VClock::default();

        for dot in iter {
            vclock.apply(dot);
        }

        vclock
    }
}

//
// Partial order
//

impl<A: Ord> PartialOrd for VClock<A> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if other.dots.iter().all(|(a, c)| self.counter_of(a) >= *c) {
            Some(Ordering::Greater)
        } else if other.dots.iter().all(|(a, c)| self.counter_of(a) <= *c) {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}

//
// TESTS
//

#[cfg(test)]
mod utest {
    use super::*;
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;
    use std::iter::FromIterator;

    impl<A: Ord + Clone + Debug + Arbitrary> Arbitrary for VClock<A> {
        fn arbitrary(g: &mut Gen) -> Self {
            let mut clock = VClock::default();

            for _ in 0..u8::arbitrary(g) % 10 {
                clock.apply(Dot::arbitrary(g));
            }

            clock
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            let mut shrunk_clocks = Vec::default();
            for dot in self.clone().into_iter() {
                let clock_without_dot: Self =
                    self.clone().into_iter().filter(|d| d != &dot).collect();

                for shrunk_dot in dot.shrink() {
                    let mut clock = clock_without_dot.clone();
                    clock.apply(shrunk_dot);
                    shrunk_clocks.push(clock);
                }

                shrunk_clocks.push(clock_without_dot);
            }

            Box::new(shrunk_clocks.into_iter())
        }
    }

    #[quickcheck]
    fn prop_default() -> bool {
        let vclock: VClock<String> = VClock::default();
        vclock.is_empty()
    }

    #[quickcheck]
    fn prop_new() -> bool {
        let vclock = VClock::<String>::new();
        vclock.is_empty()
    }

    #[quickcheck]
    fn prop_display() -> bool {
        let mut vclock = VClock::new();
        vclock.apply(vclock.inc_op("A"));
        vclock.apply(vclock.step_op("B", 10));
        eprintln!("{}", vclock);
        true
    }

    #[quickcheck]
    fn prop_vclock_display(vclock: VClock<String>) -> bool {
        eprintln!("{}", vclock);
        true
    }

    #[quickcheck]
    fn prop_debug() -> bool {
        let mut vclock = VClock::new();
        vclock.apply(vclock.inc_op("A"));
        vclock.apply(vclock.step_op("B", 10));
        eprintln!("{:?}", vclock);
        true
    }

    #[quickcheck]
    fn prop_vclock_debug(vclock: VClock<String>) -> bool {
        eprintln!("{:?}", vclock);
        true
    }

    #[quickcheck]
    fn prop_inc_op() -> bool {
        let mut vclock = VClock::new();
        let op = vclock.inc_op("A");
        vclock.apply(op);

        vclock.counter_of(&"A") == 1
    }

    #[quickcheck]
    fn prop_counter_of() -> bool {
        let mut vclock = VClock::new();
        let op = vclock.inc_op("A");
        vclock.apply(op);

        vclock.counter_of(&"A") == 1
    }

    #[quickcheck]
    fn prop_dot_of() -> bool {
        let mut vclock = VClock::new();
        let op = vclock.inc_op("A");
        vclock.apply(op.clone());

        let dot = vclock.dot_of(&"A");
        dot.actor == "A" && dot.counter == 1
    }

    #[quickcheck]
    fn prop_merge() -> bool {
        let mut a = VClock::new();
        a.apply(a.inc_op("A"));

        let mut b = VClock::new();
        b.apply(b.inc_op("B"));

        a.merge(b);

        a.counter_of(&"A") == 1 && a.counter_of(&"B") == 1
    }

    #[quickcheck]
    fn prop_into_iter() -> bool {
        let mut a = VClock::new();
        a.apply(a.inc_op("A"));

        let mut b = VClock::new();
        b.apply(b.inc_op("B"));

        a.merge(b);

        a.into_iter().count() == 2
    }

    #[quickcheck]
    fn prop_iter() -> bool {
        let mut a = VClock::new();
        a.apply(a.inc_op("A"));

        let mut b = VClock::new();
        b.apply(b.inc_op("B"));

        a.merge(b);

        a.iter().count() == 2
    }

    #[quickcheck]
    fn prop_step() -> bool {
        let mut a = VClock::new();
        a.apply(a.step_op("A", 5));

        a.counter_of(&"A") == 5
    }

    #[quickcheck]
    fn prop_partial_ord_equal() -> bool {
        let mut a = VClock::new();
        a.apply(a.step_op("A", 5));

        a.partial_cmp(&a) == Some(Ordering::Equal)
    }

    #[quickcheck]
    fn prop_partial_ord_greater() -> bool {
        let mut a = VClock::new();
        a.apply(a.step_op("A", 5));

        let mut b = VClock::new();
        b.apply(b.step_op("A", 2));

        a.partial_cmp(&b) == Some(Ordering::Greater)
    }

    #[quickcheck]
    fn prop_partial_ord_less() -> bool {
        let mut a = VClock::new();
        a.apply(a.step_op("A", 2));

        let mut b = VClock::new();
        b.apply(b.step_op("A", 4));

        a.partial_cmp(&b) == Some(Ordering::Less)
    }

    #[quickcheck]
    fn prop_partial_ord_none() -> bool {
        let mut a = VClock::new();
        a.apply(a.step_op("A", 2));
        a.apply(a.step_op("B", 4));

        let mut b = VClock::new();
        b.apply(b.step_op("A", 4));
        b.apply(b.step_op("B", 2));

        a.partial_cmp(&b) == None
    }

    #[quickcheck]
    fn prop_from_dot() -> bool {
        let dot = Dot::new("A", 10);
        let vclock = VClock::from(dot);
        vclock.dot_of("A") == Dot::new("A", 10)
    }

    #[quickcheck]
    fn prop_from_dots() -> bool {
        let dots = [Dot::new("A", 10), Dot::new("B", 1)];
        let vclock = VClock::from_iter(dots);
        vclock.counter_of(&"A") == 10
    }
}

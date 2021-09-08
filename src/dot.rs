use std::fmt;
use std::hash::{Hash, Hasher};

/// The Dot is a version marker for an actor
#[derive(Clone)]
pub struct Dot<A> {
    /// The actor identifier
    actor: A,
    /// The current version
    counter: u64,
}

impl<A> Dot<A> {
    /// Build a Dot from an actor identifier and a counter value.
    pub fn new(actor: A, counter: u64) -> Self {
        Self { actor, counter }
    }

    pub fn apply_inc(&mut self) {
        self.counter += 1;
    }
}

impl<A: Clone> Dot<A> {
    pub fn inc(&self) -> Self {
        Self {
            actor: self.actor.clone(),
            counter: self.counter + 1,
        }
    }
}

impl<A: Copy> Dot<A> {}

impl<A: PartialEq> PartialEq for Dot<A> {
    fn eq(&self, other: &Self) -> bool {
        self.actor == other.actor && self.counter == other.counter
    }
}

impl<A: Eq> Eq for Dot<A> {}

impl<A: Hash> Hash for Dot<A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.actor.hash(state);
        self.counter.hash(state);
    }
}

impl<A: PartialOrd> PartialOrd for Dot<A> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.actor == other.actor {
            self.counter.partial_cmp(&other.counter)
        } else {
            None
        }
    }
}

impl<A: fmt::Debug> fmt::Debug for Dot<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Dot")
            .field("actor", &self.actor)
            .field("counter", &self.counter)
            .finish()
    }
}

impl<A> From<(A, u64)> for Dot<A> {
    fn from(dot: (A, u64)) -> Self {
        let (actor, counter) = dot;
        Self { actor, counter }
    }
}

// TESTS

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;
    use std::cmp::Ordering;

    impl<A: Arbitrary + Clone> Arbitrary for Dot<A> {
        fn arbitrary(g: &mut Gen) -> Self {
            Dot {
                actor: A::arbitrary(g),
                counter: u64::arbitrary(g) % 50,
            }
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            let mut shrunk_dots = Vec::new();
            if self.counter > 0 {
                shrunk_dots.push(Self::new(self.actor.clone(), self.counter - 1));
            }
            Box::new(shrunk_dots.into_iter())
        }
    }

    #[quickcheck]
    fn prop_new(actor: String, counter: u64) -> bool {
        let dot = Dot::new(actor.clone(), counter);
        dot.actor == actor && dot.counter == counter
    }

    #[quickcheck]
    fn prop_clone_does_not_increment(dot: Dot<u8>) -> bool {
        dot.clone() == dot
    }

    #[quickcheck]
    fn prop_inc_increments_only_the_counter(dot: Dot<u8>) -> bool {
        dot.inc() == Dot::new(dot.actor, dot.counter + 1)
    }

    #[quickcheck]
    fn prop_apply_inc_increments_only_the_counter(dot: Dot<u8>) -> bool {
        let mut dot1 = dot.clone();
        dot1.apply_inc();
        (dot1.actor == dot.actor) && (dot1.counter == dot.counter + 1)
    }

    #[quickcheck]
    fn prop_from(actor: String, counter: u64) -> bool {
        let dot = Dot::from((actor.clone(), counter));
        dot.actor == actor && dot.counter == counter
    }

    #[quickcheck]
    fn prop_partial_order(a: Dot<u8>, b: Dot<u8>) -> bool {
        let cmp_ab = a.partial_cmp(&b);
        let cmp_ba = b.partial_cmp(&a);

        match (cmp_ab, cmp_ba) {
            (None, None) => a.actor != b.actor,
            (Some(Ordering::Less), Some(Ordering::Greater)) => {
                a.actor == b.actor && a.counter < b.counter
            }
            (Some(Ordering::Greater), Some(Ordering::Less)) => {
                a.actor == b.actor && a.counter > b.counter
            }
            (Some(Ordering::Equal), Some(Ordering::Equal)) => {
                a.actor == b.actor && a.counter == b.counter
            }
            _ => false,
        }
    }
}

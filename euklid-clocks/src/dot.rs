use std::cmp::Ordering;
use std::fmt::Debug;
use std::hash::Hash;

/// A structure for the counter attached to an actor
#[derive(Clone, Copy)]
pub struct Dot<A> {
    actor: A,
    counter: u64,
}

impl<A: Debug> Debug for Dot<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}:{}", self.actor, self.counter)
    }
}

impl<A> Dot<A> {
    /// Creates a new instance of the dot
    pub fn new(actor: A, counter: u64) -> Self {
        Self { actor, counter }
    }

    /// Applies to the current dot an increment operation.
    pub fn apply_inc_op(&mut self) {
        self.counter += 1;
    }
}

impl<A: PartialEq> PartialEq for Dot<A> {
    fn eq(&self, other: &Self) -> bool {
        self.actor == other.actor && self.counter == other.counter
    }
}

impl<A: PartialOrd> PartialOrd for Dot<A> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // We can compare dots that belong to the same actor.
        if self.actor == other.actor {
            self.counter.partial_cmp(&other.counter)
        } else {
            None
        }
    }
}

impl<A: Hash> Hash for Dot<A> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.actor.hash(state);
        self.counter.hash(state);
    }
}

impl<A> From<(A, u64)> for Dot<A> {
    fn from(dot_material: (A, u64)) -> Self {
        let (actor, counter) = dot_material;
        Self { actor, counter }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{Arbitrary, Gen};

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

    #[test]
    fn test_dot_new() {
        let dot = Dot::new(1234, 1);
        assert_eq!(1234, dot.actor);
        assert_eq!(1, dot.counter);
    }
}

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
        write!(f, "{:?}:{}", self.actor, self.counter)
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

    /// Applies to the current dot an increment with a given value.
    pub fn apply_stepup_op(&mut self, step: u64) {
        self.counter += step;
    }
}

impl<A: Clone> Dot<A> {
    /// Returns a new Dot with incremented counter
    pub fn inc(&self) -> Self {
        Self {
            actor: self.actor.clone(),
            counter: self.counter + 1,
        }
    }

    /// Returns a new Dot with increased counter
    pub fn stepup(&self, step: u64) -> Self {
        Self {
            actor: self.actor.clone(),
            counter: self.counter + step,
        }
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
    use quickcheck_macros::quickcheck;
    use std::{cmp::Ordering, collections::hash_map::DefaultHasher, hash::Hasher};

    impl<A: Arbitrary + Clone> Arbitrary for Dot<A> {
        fn arbitrary(g: &mut Gen) -> Self {
            Dot {
                actor: A::arbitrary(g),
                counter: u64::arbitrary(g) % 50,
            }
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            let mut shrunk_dots = Vec::new();
            let mut counter = self.counter;
            loop {
                shrunk_dots.push(Self::new(self.actor.clone(), counter));
                if counter == 0 {
                    break;
                }

                counter -= 1;
            }

            Box::new(shrunk_dots.into_iter())
        }
    }

    #[quickcheck]
    fn test_shrink(actor: i32, counter: usize) -> bool {
        let counter = counter % 10;
        let dot = Dot::new(actor, counter as u64);
        let xs = dot.shrink();
        assert_eq!(counter + 1, xs.count());
        true
    }

    #[quickcheck]
    fn test_new(actor: i32, counter: u64) -> bool {
        let dot = Dot::new(actor, counter);
        assert_eq!(actor, dot.actor);
        assert_eq!(counter, dot.counter);
        true
    }

    #[quickcheck]
    fn test_debug(dot: Dot<i32>) -> bool {
        let s = format!("{:?}", dot);
        assert!(!s.is_empty());
        true
    }

    #[quickcheck]
    fn test_clone(dot: Dot<i32>) -> bool {
        let dot1 = dot;
        assert_eq!(dot, dot1);
        true
    }

    #[quickcheck]
    fn test_eq_diff_counter(actor: i32, counter: u32) -> bool {
        let dot = Dot::new(actor, counter as u64);
        let dot1 = Dot::new(actor, (counter as u64) + 1);
        assert!(!dot.eq(&dot1));
        true
    }

    #[quickcheck]
    fn test_eq_diff_actor(actor: i32, counter: u64) -> bool {
        let dot = Dot::new(actor as i64, counter);
        let dot1 = Dot::new((actor as i64) + 1, counter);
        assert!(!dot.eq(&dot1));
        true
    }

    #[quickcheck]
    fn test_pord_counter(actor: i32, counter: u32) -> bool {
        let dot = Dot::new(actor, counter as u64);
        let dot1 = Dot::new(actor, (counter as u64) + 1);
        assert_eq!(Some(Ordering::Less), dot.partial_cmp(&dot1));
        true
    }

    #[quickcheck]
    fn test_pord_diff_actor(actor: i16, counter: u64) -> bool {
        let dot = Dot::new(actor as i32, counter);
        let dot1 = Dot::new((actor as i32) + 1, counter);
        dot.partial_cmp(&dot1).is_none()
    }

    #[quickcheck]
    fn test_hash(dot: Dot<i32>) -> bool {
        let mut hasher = DefaultHasher::new();
        dot.hash(&mut hasher);
        hasher.finish() != 0
    }

    #[quickcheck]
    fn test_from(actor: i32, counter: u64) -> bool {
        let dot: Dot<i32> = (actor, counter).into();
        dot.actor == actor && dot.counter == counter
    }

    #[quickcheck]
    fn test_apply_inc_op(actor: i32, counter: u32) -> bool {
        let mut dot = Dot::new(actor, counter as u64);
        dot.apply_inc_op();
        dot.actor == actor && dot.counter == (counter as u64) + 1
    }

    #[quickcheck]
    fn test_apply_stepup_op(actor: i32, counter: u16, step: u16) -> bool {
        let mut dot = Dot::new(actor, counter as u64);
        dot.apply_stepup_op(step as u64);
        dot.actor == actor && dot.counter == (counter as u64) + (step as u64)
    }

    #[quickcheck]
    fn test_inc(actor: i32, counter: u32) -> bool {
        let dot = Dot::new(actor, counter as u64);
        let dot1 = dot.inc();
        dot1.actor == actor && dot1.counter == (counter as u64) + 1
    }

    #[quickcheck]
    fn test_stepup(actor: i32, counter: u16, step: u16) -> bool {
        let dot = Dot::new(actor, counter as u64);
        let dot1 = dot.stepup(step as u64);
        dot1.actor == actor && dot1.counter == (counter as u64) + (step as u64)
    }
}

use std::cmp::Ordering;
use std::fmt::Debug;

/// A structure for the counter attached to an actor
pub struct Dot<A> {
    actor: A,
    counter: u128,
}

impl<A: Debug> Debug for Dot<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}:{}", self.actor, self.counter)
    }
}

impl<A> Dot<A> {
    /// Creates a new instance of the dot
    pub fn new(actor: A, counter: u128) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot_new() {
        let dot = Dot::new(1234, 1);
        assert_eq!(1234, dot.actor);
        assert_eq!(1, dot.counter);
    }
}

use super::CausalOrd;

use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::{Add, AddAssign, BitOrAssign};

/// The dot structure pairs an actor with a counter. It can be
/// used to implement vector clocks, dotted vector clocks, and
/// other CRDTs structures.
///
/// # Example
///
/// ```rust
/// use euklid_clocks::*;
///
/// let dot1: Dot<String> = ("A".to_string(), 1).into();
/// println!("dot1={:?}", dot1);
/// let dot2 = dot1.incr();
/// println!("dot2={:?}", dot2);
///
/// assert!(dot1.is_descendant(&dot1));
/// assert!(dot2.is_descendant(&dot1));
/// assert!(dot2.is_dominating(&dot1));
///
/// let mut dot3 = dot2 + 1;
/// dot3 += 10;
/// println!("dot3={:?}", dot3);
///
/// dot3 |= 30;
/// assert_eq!(30, dot3.counter);
/// ```
#[derive(Clone, Copy)]
pub struct Dot<A> {
    /// The actor identifier
    pub actor: A,
    /// The current counter value
    pub counter: u64,
}

//
// Public functions
//

impl<A> Dot<A> {
    /// Creates a new instance of the dot
    pub fn new(actor: A, counter: u64) -> Self {
        Self { actor, counter }
    }

    /// Applies to the current dot an increment operation.
    pub fn apply_inc_op(&mut self) {
        self.apply_step_op(1);
    }

    /// Applies to the current dot an increment with a given step operation.
    pub(crate) fn apply_step_op(&mut self, s: u64) {
        self.counter += s;
    }
}

impl<A: Clone> Dot<A> {
    /// Returns a new Dot with incremented counter
    pub fn incr(&self) -> Self {
        self.step(1)
    }

    /// Returns a new Dot with an incremented counter with a given step.
    pub fn step(&self, s: u64) -> Self {
        Self {
            actor: self.actor.clone(),
            counter: self.counter + s,
        }
    }
}

//
// Operations
//

impl<A: Clone> Add<u64> for Dot<A> {
    type Output = Dot<A>;

    fn add(self, rhs: u64) -> Self::Output {
        self.step(rhs)
    }
}

impl<A> AddAssign<u64> for Dot<A> {
    fn add_assign(&mut self, rhs: u64) {
        self.apply_step_op(rhs);
    }
}

impl<A> BitOrAssign<u64> for Dot<A> {
    fn bitor_assign(&mut self, rhs: u64) {
        if self.counter < rhs {
            self.counter = rhs;
        }
    }
}

impl<A: PartialEq> BitOrAssign for Dot<A> {
    fn bitor_assign(&mut self, rhs: Self) {
        if self.actor == rhs.actor && self.counter < rhs.counter {
            self.counter = rhs.counter;
        }
    }
}

//
// Causal ordering
//

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

impl<A: PartialOrd> CausalOrd for Dot<A> {}

//
// Froms
//

impl<A> From<(A, u64)> for Dot<A> {
    fn from(pair: (A, u64)) -> Self {
        let (actor, counter) = pair;
        Self { actor, counter }
    }
}

impl<A> From<A> for Dot<A> {
    fn from(actor: A) -> Self {
        Self { actor, counter: 0 }
    }
}

//
// Formatting
//

impl<A: Debug> Debug for Dot<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}:{}", self.actor, self.counter)
    }
}

//
// Tests
//

#[cfg(test)]
mod tests {
    use crate::CausalOrdering;

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
    fn test_cmp_precedes(actor: i32, counter: u32) -> bool {
        let dot = Dot::new(actor, counter as u64);
        let dot1 = Dot::new(actor, (counter as u64) + 1);
        matches!(dot.causal_cmp(&dot1), CausalOrdering::Precede)
    }

    #[quickcheck]
    fn test_is_descendant_eq(actor: i32, counter: u32) -> bool {
        let dot = Dot::new(actor, counter as u64);
        dot.is_descendant(&dot)
    }

    #[quickcheck]
    fn test_is_descendant_succ(actor: i32, counter: u32) -> bool {
        let dot = Dot::new(actor, counter as u64);
        let dot1 = Dot::new(actor, (counter as u64) + 1);
        dot1.is_descendant(&dot)
    }

    #[quickcheck]
    fn test_is_dominating(actor: i32, counter: u32) -> bool {
        let dot = Dot::new(actor, counter as u64);
        let dot1 = Dot::new(actor, (counter as u64) + 1);
        dot1.is_dominating(&dot)
    }

    #[quickcheck]
    fn test_is_concurrent(actor: i16, counter: u64) -> bool {
        let a = Dot::new(actor as i32, counter);
        let b = Dot::new((actor as i32) + 1, counter);
        b.is_concurrent(&a)
    }

    #[quickcheck]
    fn test_is_ancestor(actor: i32, counter: u32) -> bool {
        let dot = Dot::new(actor, counter as u64);
        let dot1 = Dot::new(actor, (counter as u64) + 1);
        dot.is_ancestor(&dot1)
    }

    #[quickcheck]
    fn test_from_pair(actor: i32, counter: u64) -> bool {
        let dot: Dot<i32> = (actor, counter).into();
        dot.actor == actor && dot.counter == counter
    }

    #[quickcheck]
    fn test_from_actor(actor: i32) -> bool {
        let dot: Dot<i32> = actor.into();
        dot.actor == actor && dot.counter == 0
    }

    #[quickcheck]
    fn test_apply_inc_op(actor: i32, counter: u32) -> bool {
        let mut dot = Dot::new(actor, counter as u64);
        dot.apply_inc_op();
        dot.actor == actor && dot.counter == (counter as u64) + 1
    }

    #[quickcheck]
    fn test_apply_step_op(actor: i32, counter: u32) -> bool {
        let mut dot = Dot::new(actor, counter as u64);
        dot.apply_step_op(20);
        dot.actor == actor && dot.counter == (counter as u64) + 20
    }

    #[quickcheck]
    fn test_incr(actor: i32, counter: u32) -> bool {
        let dot = Dot::new(actor, counter as u64);
        let dot1 = dot.incr();
        dot1.actor == actor && dot1.counter == (counter as u64) + 1
    }

    #[quickcheck]
    fn test_step(actor: i32, counter: u32) -> bool {
        let dot = Dot::new(actor, counter as u64);
        let dot1 = dot.step(10);
        dot1.actor == actor && dot1.counter == (counter as u64) + 10
    }

    #[quickcheck]
    fn test_add(actor: i32, counter: u32) -> bool {
        let dot = Dot::new(actor, counter as u64);
        let dot1 = dot + 1;
        dot1.actor == actor && dot1.counter == (counter as u64) + 1
    }

    #[quickcheck]
    fn test_add_assign(actor: i32, counter: u32) -> bool {
        let mut dot = Dot::new(actor, counter as u64);
        dot += 1;
        dot.counter == (counter as u64) + 1
    }

    #[quickcheck]
    fn test_bitor_assign_ok(actor: i32, counter: u32) -> bool {
        let mut dot = Dot::new(actor, counter as u64);
        dot |= (counter as u64) + 10;
        dot.counter == (counter as u64) + 10
    }

    #[quickcheck]
    fn test_bitor_assign_no(actor: i32, counter: u32) -> bool {
        let mut dot = Dot::new(actor, (counter as u64) + 10);
        dot |= counter as u64;
        dot.counter == (counter as u64) + 10
    }

    #[quickcheck]
    fn test_dot_bitorassign_ok(actor: i32, counter: u32) -> bool {
        let mut dot = Dot::new(actor, counter as u64);
        let dot1 = Dot::new(actor, counter as u64 + 10);
        dot |= dot1;
        dot.counter == (counter as u64) + 10
    }

    #[quickcheck]
    fn test_dot_bitorassign_no_actor(actor: i16, counter: u32) -> bool {
        let mut dot = Dot::new(actor as i32, counter as u64);
        let dot1 = Dot::new(actor as i32 + 1, counter as u64 + 10);
        dot |= dot1;
        dot.counter == counter as u64
    }

    #[quickcheck]
    fn test_dot_bitorassign_no_counter(actor: i32, counter: u32) -> bool {
        let mut dot = Dot::new(actor, counter as u64 + 10);
        let dot1 = Dot::new(actor, counter as u64);
        dot |= dot1;
        dot.counter == (counter as u64) + 10
    }
}

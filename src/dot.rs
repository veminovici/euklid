use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::{Add, Sub};
use std::ops::{AddAssign, SubAssign};

use crate::Actor;
use crate::CausalityOrd;
use crate::Counter;
use crate::One;
use crate::UpdateAssign;
use crate::Zero;

/// `Dot` instance for an actor.
#[derive(Copy)]
pub struct Dot<A: Actor, C: Counter> {
    pub(crate) actor: A,
    pub(crate) counter: C,
}

//
// Constructor traits
//

impl<A: Actor, C: Counter> Default for Dot<A, C> {
    fn default() -> Self {
        Self::one()
    }
}

impl<A: Actor, C: Counter> From<(A, C)> for Dot<A, C> {
    fn from(ac: (A, C)) -> Self {
        Self {
            actor: ac.0,
            counter: ac.1,
        }
    }
}

impl<A: Actor, C: Counter> Clone for Dot<A, C> {
    fn clone(&self) -> Self {
        Self {
            actor: self.actor.clone(),
            counter: self.counter.incr(),
        }
    }
}

//
// Formatting traits
//

impl<A: Actor + std::fmt::Display, C: Counter + std::fmt::Display> std::fmt::Display for Dot<A, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.actor, self.counter)
    }
}

//
// Order traits
//

impl<A: Actor, C: Counter> PartialEq for Dot<A, C> {
    fn eq(&self, other: &Self) -> bool {
        self.actor == other.actor && self.counter == other.counter
    }
}

impl<A: Actor, C: Counter> PartialOrd for Dot<A, C> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.actor == other.actor {
            self.counter.partial_cmp(&other.counter)
        } else {
            None
        }
    }
}

impl<A: Actor, C: Counter> CausalityOrd for Dot<A, C> {}

//
// Identities
//

impl<A: Actor, C: Counter> Zero for Dot<A, C> {
    fn zero() -> Self {
        Self {
            actor: A::zero(),
            counter: C::zero(),
        }
    }
}

impl<A: Actor, C: Counter> One for Dot<A, C> {
    fn one() -> Self {
        Self {
            actor: A::zero(),
            counter: C::one(),
        }
    }
}

//
// Operations
//

impl<A: Actor, C: Counter> Add for Dot<A, C> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            actor: self.actor,
            counter: self.counter + rhs.counter,
        }
    }
}

impl<A: Actor, C: Counter> AddAssign for Dot<A, C> {
    fn add_assign(&mut self, rhs: Self) {
        self.counter += rhs.counter;
    }
}

impl<A: Actor, C: Counter> Sub for Dot<A, C> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            actor: self.actor,
            counter: self.counter - rhs.counter,
        }
    }
}

impl<A: Actor, C: Counter> SubAssign for Dot<A, C> {
    fn sub_assign(&mut self, rhs: Self) {
        self.counter -= rhs.counter;
    }
}

impl<A: Actor, C: Counter> UpdateAssign for Dot<A, C> {
    fn upd_assign(&mut self, rhs: Self) {
        self.counter = rhs.counter;
    }
}

impl<A: Actor, C: Counter> UpdateAssign<C> for Dot<A, C> {
    fn upd_assign(&mut self, rhs: C) {
        self.counter = rhs;
    }
}

//
// Hashing
//

impl<A: Actor + Hash, C: Counter + Hash> Hash for Dot<A, C> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.actor.hash(state);
        self.counter.hash(state);
    }
}

//
// Implementation
//

impl<A: Actor, C: Counter> Dot<A, C> {
    /// Constructs a new `Dot` instance.
    pub fn new(actor: A, counter: C) -> Self {
        Self { actor, counter }
    }

    /// Constructs a new `Dot` instance for a given actor with the counter set to zero.
    pub fn new_zero(actor: A) -> Self {
        Self {
            actor,
            counter: C::zero(),
        }
    }

    /// Constructs a new `Dot` instance for a given actor with the counter set to one.
    pub fn new_one(actor: A) -> Self {
        Self {
            actor,
            counter: C::one(),
        }
    }

    /// Construsts a new `Dot` instance which has the incremented counter.
    pub fn incr(&self) -> Self {
        Self {
            actor: self.actor,
            counter: self.counter.incr(),
        }
    }

    /// Apply the increment operation.
    pub fn incr_assign(&mut self) {
        self.counter.incr_assign();
    }

    /// Constructs a new `Dot` instance which ahs the decremented counter.
    pub fn decr(&self) -> Self {
        Self {
            actor: self.actor,
            counter: self.counter.decr(),
        }
    }

    /// Apply the decrement operation.
    pub fn decr_assign(&mut self) {
        self.counter.decr_assign();
    }

    /// Constructs a new `Dot` instance with the specified counter value.
    pub fn upd(&self, counter: C) -> Self {
        Self {
            actor: self.actor,
            counter,
        }
    }
}

//
// DotRange - a range of dots for a given actor.
//

/// Represents a range of `dot` values for an actor.
pub struct DotRange<A: Actor, C: Counter> {
    pub(crate) actor: A,
    pub(crate) range: core::ops::Range<C>,
}

impl<A: Actor + Display, C: Counter + Display> Display for DotRange<A, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.({}..{})",
            self.actor, self.range.start, self.range.end
        )
    }
}

impl<A: Actor + Debug, C: Counter + Debug> Debug for DotRange<A, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}.({:?}..{:?})",
            self.actor, self.range.start, self.range.end
        )
    }
}

impl<A: Actor + Debug + Display, C: Counter + Debug + Display> std::error::Error
    for DotRange<A, C>
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Causality;

    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;

    impl<A: Actor + Arbitrary, C: Counter + Arbitrary> Arbitrary for Dot<A, C> {
        fn arbitrary(g: &mut Gen) -> Self {
            Dot {
                actor: A::arbitrary(g),
                counter: C::arbitrary(g),
            }
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            let mut shrunk_dots = Vec::new();
            if self.counter > C::zero() {
                let dot = Self {
                    actor: self.actor,
                    counter: self.counter.decr(),
                };

                shrunk_dots.push(dot);
            }

            Box::new(shrunk_dots.into_iter())
        }
    }

    macro_rules! test_new {
        ($t:ty, $fnnew:ident, $fnnewzero:ident, $fnnewone:ident) => {
            #[quickcheck]
            fn $fnnew(actor: i8, counter: $t) -> bool {
                let dot = Dot::new(actor, counter);
                actor == dot.actor && counter == dot.counter
            }
            #[quickcheck]
            fn $fnnewzero(actor: i8) -> bool {
                let dot: Dot<i8, $t> = Dot::new_zero(actor);
                actor == dot.actor && dot.counter == <$t>::zero()
            }
            #[quickcheck]
            fn $fnnewone(actor: i8) -> bool {
                let dot: Dot<i8, $t> = Dot::new_one(actor);
                actor == dot.actor && dot.counter == <$t>::one()
            }
        };
    }

    test_new!(usize, usize_new, usize_new_zero, usize_new_one);
    test_new!(u8, u8_new, u8_new_zero, u8_new_one);
    test_new!(u16, u16_new, u16_new_zero, u16_new_one);
    test_new!(u32, u32_new, u32_new_zero, u32_new_one);
    test_new!(u64, u64_new, u64_new_zero, u64_new_one);
    test_new!(u128, u128_new, u128_new_zero, u128_new_one);

    test_new!(i8, i8_new, i8_new_zero, i8_new_one);
    test_new!(i16, i16_new, i16_new_zero, i16_new_one);
    test_new!(i32, i32_new, i32_new_zero, i32_new_one);
    test_new!(i64, i64_new, i64_new_zero, i64_new_one);
    test_new!(i128, i128_new, i128_new_zero, i128_new_one);

    macro_rules! test_from {
        ($t:ty, $fnfrom:ident) => {
            #[quickcheck]
            fn $fnfrom(actor: i8, counter: $t) -> bool {
                let dot: Dot<i8, $t> = (actor, counter).into();
                actor == dot.actor && counter == dot.counter
            }
        };
    }

    test_from!(usize, usize_from);
    test_from!(u8, u8_from);
    test_from!(u16, u16_from);
    test_from!(u32, u32_from);
    test_from!(u64, u64_from);
    test_from!(u128, u128_from);

    test_from!(i8, i8_from);
    test_from!(i16, i16_from);
    test_from!(i32, i32_from);
    test_from!(i64, i64_from);
    test_from!(i128, i128_from);

    macro_rules! test_causality {
        ($t:ty, $fneq:ident, $fnprecede:ident, $fnsucceed:ident, $fnconcurrent:ident) => {
            #[quickcheck]
            fn $fneq(actor: i8, count: $t) -> bool {
                let dot1: Dot<i8, $t> = (actor, count).into();
                let dot2: Dot<i8, $t> = (actor, count).into();
                dot1.causality_cmp(&dot2) == Causality::Equal
            }

            #[quickcheck]
            fn $fnprecede(actor: i8, count: $t) -> bool {
                let dot1: Dot<i8, $t> = (actor, count / 2).into();
                let dot2: Dot<i8, $t> = (actor, count / 2 + 1).into();
                dot1.causality_cmp(&dot2) == Causality::Precede
            }

            #[quickcheck]
            fn $fnsucceed(actor: i8, count: $t) -> bool {
                let dot1: Dot<i8, $t> = (actor, count / 2).into();
                let dot2: Dot<i8, $t> = (actor, count / 2 + 1).into();
                dot2.causality_cmp(&dot1) == Causality::Succeed
            }
            #[quickcheck]
            fn $fnconcurrent(actor: i8, count: $t) -> bool {
                let dot1: Dot<i8, $t> = (actor / 2, count / 2).into();
                let dot2: Dot<i8, $t> = (actor / 2 + 1, count / 2 + 1).into();
                dot2.causality_cmp(&dot1) == Causality::Concurrent
            }
        };
    }

    test_causality!(
        usize,
        usize_causal_eq,
        usize_causal_precede,
        usize_causal_succeed,
        usize_causal_concurrent
    );
    test_causality!(
        u8,
        u8_causal_eq,
        u8_causal_precede,
        u8_causal_succeed,
        u8_causal_concurrent
    );
    test_causality!(
        u16,
        u16_causal_eq,
        u16_causal_precede,
        u16_causal_succeed,
        u16_causal_concurrent
    );
    test_causality!(
        u32,
        u32_causal_eq,
        u32_causal_precede,
        us32_causal_succeed,
        u32_causal_concurrent
    );
    test_causality!(
        u64,
        u64_causal_eq,
        u64_causal_precede,
        u64_causal_succeed,
        u64_causal_concurrent
    );
    test_causality!(
        u128,
        u128_causal_eq,
        u128_causal_precede,
        u128_causal_succeed,
        u128_causal_concurrent
    );

    macro_rules! test_incr_assign {
        ($t:ty, $fnincr:ident, $fnincr_assign:ident) => {
            #[quickcheck]
            fn $fnincr(actor: i8, counter: $t) -> bool {
                let dot: Dot<i8, $t> = (actor, counter / 2).into();
                let dot1 = dot.incr();
                actor == dot1.actor && counter / 2 + 1 == dot1.counter
            }

            #[quickcheck]
            fn $fnincr_assign(actor: i8, counter: $t) -> bool {
                let mut dot: Dot<i8, $t> = (actor, counter / 2).into();
                dot.incr_assign();
                actor == dot.actor && counter / 2 + 1 == dot.counter
            }
        };
    }

    test_incr_assign!(usize, usize_incr, usize_incr_assign);
    test_incr_assign!(u8, u8_incr, u8_incr_assign);
    test_incr_assign!(u16, u16_incr, u16_incr_assign);
    test_incr_assign!(u32, u32_incr, u32_incr_assign);
    test_incr_assign!(u64, u64_incr, u64_incr_assign);
    test_incr_assign!(u128, u128_incr, u128_incr_assign);

    macro_rules! test_decr_assign {
        ($t:ty, $fndecr:ident, $fndecr_assign:ident) => {
            #[quickcheck]
            fn $fndecr(actor: i8, counter: $t) -> bool {
                let dot: Dot<i8, $t> = (actor, counter / 2 + 10).into();
                let dot1 = dot.decr();
                actor == dot1.actor && counter / 2 + 9 == dot1.counter
            }

            #[quickcheck]
            fn $fndecr_assign(actor: i8, counter: $t) -> bool {
                let mut dot: Dot<i8, $t> = (actor, counter / 2 + 10).into();
                dot.decr_assign();
                actor == dot.actor && counter / 2 + 9 == dot.counter
            }
        };
    }

    test_decr_assign!(usize, usize_decr, usize_decr_assign);
    test_decr_assign!(u8, u8_decr, u8_decr_assign);
    test_decr_assign!(u16, u16_decr, u16_decr_assign);
    test_decr_assign!(u32, u32_decr, u32_decr_assign);
    test_decr_assign!(u64, u64_decr, u64_decr_assign);
    test_decr_assign!(u128, u128_decr, u128_decr_assign);

    macro_rules! test_upd_assign {
        ($t:ty, $fnupd:ident) => {
            #[quickcheck]
            fn $fnupd(actor: i8, count1: $t, count2: $t) -> bool {
                let mut dot: Dot<i8, $t> = (actor, count1).into();
                dot.upd_assign(count2);
                actor == dot.actor && count2 == dot.counter
            }
        };
    }

    test_upd_assign!(usize, usize_upd_assign);
    test_upd_assign!(u8, u8_upd_assign);
    test_upd_assign!(u16, u16_upd_assign);
    test_upd_assign!(u32, u32_upd_assign);
    test_upd_assign!(u64, u64_upd_assign);
    test_upd_assign!(u128, u128_upd_assign);

    #[quickcheck]
    fn test_display(actor: i8, count: usize) -> bool {
        let dot: Dot<i8, usize> = (actor, count).into();
        format!("{}", dot).len() != 0
    }

    macro_rules! test_clone {
        ($t:ty, $fnclone:ident) => {
            #[quickcheck]
            fn $fnclone(actor: i8, counter: $t) -> bool {
                let dot: Dot<i8, $t> = (actor, counter / 2).into();
                let dot1 = dot.clone();
                actor == dot1.actor && counter / 2 + 1 == dot1.counter
            }
        };
    }

    test_clone!(usize, usize_clone);
    test_clone!(u8, u8_clone);
    test_clone!(u16, u16_clone);
    test_clone!(u32, u32_clone);
    test_clone!(u64, u64_clone);
    test_clone!(u128, u128_clone);
}

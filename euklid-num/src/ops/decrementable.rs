use crate::identities::One;
use std::ops::SubAssign;

/// Trait that defines the decrementable behavior.
pub trait Decrementable: One + SubAssign {
    /// Increments in-place a given value.
    #[inline]
    fn decr(&mut self) {
        *self -= Self::one();
    }
}

macro_rules! decr_impl {
    ($t:ty) => {
        impl Decrementable for $t {}
    };
}

decr_impl!(usize);
decr_impl!(u8);
decr_impl!(u16);
decr_impl!(u32);
decr_impl!(u64);
decr_impl!(u128);

decr_impl!(isize);
decr_impl!(i8);
decr_impl!(i16);
decr_impl!(i32);
decr_impl!(i64);
decr_impl!(i128);

decr_impl!(f32);
decr_impl!(f64);

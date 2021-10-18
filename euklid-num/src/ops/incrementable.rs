use crate::identities::One;
use std::ops::AddAssign;

/// Trait that defines the incrementable behavior.
pub trait Incrementable: One + AddAssign {
    /// Increments in-place a given value.
    #[inline]
    fn incr(&mut self) {
        *self += Self::one();
    }
}

macro_rules! incr_impl {
    ($t:ty) => {
        impl Incrementable for $t {}
    };
}

incr_impl!(usize);
incr_impl!(u8);
incr_impl!(u16);
incr_impl!(u32);
incr_impl!(u64);
incr_impl!(u128);

incr_impl!(isize);
incr_impl!(i8);
incr_impl!(i16);
incr_impl!(i32);
incr_impl!(i64);
incr_impl!(i128);

incr_impl!(f32);
incr_impl!(f64);

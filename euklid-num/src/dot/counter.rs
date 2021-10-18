use crate::identities::Zero;
use crate::ops::Incrementable;

/// Defines an ever growing counter.
pub trait GrowingCounter: Incrementable + PartialOrd + Zero {
    /// Creates the zero value for the counter
    fn zero() -> Self {
        Zero::zero()
    }
}

macro_rules! grow_counter_impl {
    ($t:ty) => {
        impl GrowingCounter for $t {}
    };
}

grow_counter_impl!(usize);
grow_counter_impl!(u8);
grow_counter_impl!(u16);
grow_counter_impl!(u32);
grow_counter_impl!(u64);
grow_counter_impl!(u128);

grow_counter_impl!(isize);
grow_counter_impl!(i8);
grow_counter_impl!(i16);
grow_counter_impl!(i32);
grow_counter_impl!(i64);
grow_counter_impl!(i128);

grow_counter_impl!(f32);
grow_counter_impl!(f64);

/// An actor identity
pub trait ActorId: PartialEq {}

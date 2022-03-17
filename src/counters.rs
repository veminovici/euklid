use crate::Counter;
use crate::One;
use crate::Zero;

macro_rules! counter_impl {
    ($t:ty) => {
        impl Zero for $t {
            fn zero() -> Self {
                0 as $t
            }
        }

        impl One for $t {
            fn one() -> Self {
                1 as $t
            }
        }

        impl Counter for $t {}
    };
}

counter_impl!(usize);
counter_impl!(u8);
counter_impl!(u16);
counter_impl!(u32);
counter_impl!(u64);
counter_impl!(u128);

counter_impl!(i8);
counter_impl!(i16);
counter_impl!(i32);
counter_impl!(i64);
counter_impl!(i128);

/// Trait which defines the zero identity.
pub trait Zero: Sized {
    /// Returns the zero identity for the type
    fn zero() -> Self;

    /// Sets to zero a value.
    fn set_zero(&mut self) {
        *self = Self::zero();
    }

    /// Determines if a value is zero.
    fn is_zero(&self) -> bool;
}

macro_rules! zero_impl {
    ($t:ty, $v:expr) => {
        impl Zero for $t {
            #[inline]
            fn zero() -> $t {
                $v
            }
            #[inline]
            fn is_zero(&self) -> bool {
                *self == $v
            }
        }
    };
}

macro_rules! zero_float_impl {
    ($t:ty, $v:expr) => {
        impl Zero for $t {
            #[inline]
            fn zero() -> $t {
                $v
            }

            #[inline]
            fn is_zero(&self) -> bool {
                (*self - Self::zero()).abs() < Self::EPSILON
            }
        }
    };
}

//
// Num types implement Zero trait.
//

zero_impl!(usize, 0);
zero_impl!(u8, 0);
zero_impl!(u16, 0);
zero_impl!(u32, 0);
zero_impl!(u64, 0);
zero_impl!(u128, 0);

zero_impl!(isize, 0);
zero_impl!(i8, 0);
zero_impl!(i16, 0);
zero_impl!(i32, 0);
zero_impl!(i64, 0);
zero_impl!(i128, 0);

zero_float_impl!(f32, 0.0);
zero_float_impl!(f64, 0.0);

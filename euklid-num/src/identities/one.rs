/// Trait that defines the one identity.
pub trait One: Sized {
    /// Retursn one zero
    fn one() -> Self;

    /// Sets the value to one
    fn set_one(&mut self) {
        *self = Self::one();
    }

    /// Determnines if a value is one.
    fn is_one(&self) -> bool;
}

macro_rules! one_impl {
    ($t:ty, $v:expr) => {
        impl One for $t {
            #[inline]
            fn one() -> $t {
                $v
            }
            #[inline]
            fn is_one(&self) -> bool {
                *self == $v
            }
        }
    };
}

macro_rules! one_float_impl {
    ($t:ty, $v:expr) => {
        impl One for $t {
            #[inline]
            fn one() -> $t {
                $v
            }

            #[inline]
            fn is_one(&self) -> bool {
                (*self - Self::one()).abs() < Self::EPSILON
            }
        }
    };
}

one_impl!(usize, 1);
one_impl!(u8, 1);
one_impl!(u16, 1);
one_impl!(u32, 1);
one_impl!(u64, 1);
one_impl!(u128, 1);

one_impl!(isize, 1);
one_impl!(i8, 1);
one_impl!(i16, 1);
one_impl!(i32, 1);
one_impl!(i64, 1);
one_impl!(i128, 1);

one_float_impl!(f32, 1.0);
one_float_impl!(f64, 1.0);

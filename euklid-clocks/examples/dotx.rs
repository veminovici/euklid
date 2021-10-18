use std::ops::{Add, AddAssign, Sub, SubAssign};

//
// Zero trait
//

pub trait Zero: Sized {
    fn zero() -> Self;

    fn set_zero(&mut self) {
        *self = Self::zero();
    }

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

zero_impl!(f32, 0.0);
zero_impl!(f64, 0.0);

//
// One trait
//

pub trait One: Sized {
    fn one() -> Self;

    fn set_one(&mut self) {
        *self = Self::one();
    }

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

one_impl!(f32, 1.0);
one_impl!(f64, 1.0);

//
// Actor trait
//

pub trait Actor: PartialEq {}

macro_rules! actor_impl {
    ($t:ty) => {
        impl Actor for $t {}
    };
}

actor_impl!(usize);
actor_impl!(u8);
actor_impl!(u16);
actor_impl!(u32);
actor_impl!(u64);
actor_impl!(u128);

//
// Incrementable trait
//

pub trait Incrementable: One + AddAssign {
    fn incr(&mut self) {
        *self += Self::one();
    }
}

pub trait Decrementable: One + SubAssign {
    fn decre(&mut self) {
        *self -= Self::one();
    }
}

//
// Counter
//

trait Counter: PartialOrd + Zero + CounterOps {
    fn testing() -> Self;
}

trait CounterOps<Rhs = Self, Output = Self>:
    Add<Rhs, Output = Output> + AddAssign<Rhs> + Sub<Rhs, Output = Output> + SubAssign<Rhs>
{
}

impl<T, Rhs, Output> CounterOps<Rhs, Output> for T where
    T: Add<Rhs, Output = Output> + AddAssign<Rhs> + Sub<Rhs, Output = Output> + SubAssign<Rhs>
{
}

fn main() {
    println!("Testing even a better dot");
    //<i32 as Counter>::testing();
}

use crate::Actor;

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

actor_impl!(i8);
actor_impl!(i16);
actor_impl!(i32);
actor_impl!(i64);
actor_impl!(i128);

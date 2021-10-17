# ![rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white) Simplee...Euklid... 

Just another rust crate, this one implements CRDTs things.

[![CI Pipeline](https://github.com/veminovici/euklid/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/veminovici/euklid/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/veminovici/euklid/branch/main/graph/badge.svg?token=IKPMJE7FHB)](https://codecov.io/gh/veminovici/euklid)
[![Coverage Status](https://coveralls.io/repos/github/veminovici/euklid/badge.svg)](https://coveralls.io/github/veminovici/euklid)
[![Tag](https://img.shields.io/github/tag/veminovici/euklid)](https://github.com/veminovici/euklid)
[![Last commit](https://img.shields.io/github/last-commit/veminovici/euklid)](https://github.com/veminovici/euklid)
[![Repo size](https://img.shields.io/github/repo-size/veminovici/euklid)](https://github.com/veminovici/euklid)

[![Github Actions](https://buildstats.info/github/chart/veminovici/euklid)](https://github.com/veminovici/euklid)

<br/>

### Causality
The crate defines the **CausalOrdering** enumeration which has 4 values:

```rust
pub enum CausalOrdering {
    /// A causal ordering where a compared dot precedes another.
    Precede,
    /// A causal ordering where a compared dot is equal to another.
    Equal,
    /// A causal ordering where a compared dot succeeds another.
    Succeed,
    /// A causal ordering where a compared dot is concurrent to another.
    Concurrent,
}
```

The crate also defines **CausalOrd** trait which along with the **CasualOrdering** allows the caller
to determine if there is any causality between two different instances of the **CausalOrd**. The trait defines

```rust
pub trait CausalOrd: PartialOrd<Self> {
    /// This method returns a causal ordering between `self` and `other` values if one exists.
    fn causal_cmp(&self, other: &Self) -> CausalOrdering;

    /// This method tests succeed (for `self` and `other`)
    fn is_dominating(&self, other: &Self) -> bool;

    /// This method tests succeed and identical (for `self` and `other`)
    fn is_descendant(&self, other: &Self) -> bool;

    /// This method tests preceds (for `self` and `other`)
    fn is_ancestor(&self, other: &Self) -> bool;

    /// This method tests concurrent (for `self` and `other`)
    fn is_concurrent(&self, other: &Self) -> bool;
}
```

If a structure implements the **std::cmn::PartialOrd**, you can implement the **CausalOrd**. You dont have to impement any of its
functions, since all of them have default implementations.

### Dot
The [Dot](https://github.com/veminovici/euklid/blob/main/euklid-clocks/src/dot.rs) is implementing a dot for clocks.
```rust
use euklid_clocks::*;

// Create a dot from a pair
let dot1: Dot<String> = ("A".to_string(), 1).into();
println!("dot1={:?}", dot1);

// Increment the dot
let dot2 = dot1.incr();
println!("dot2={:?}", dot2);

// Assert against causal properties
assert!(dot1.is_descendant(&dot1));
assert!(dot2.is_descendant(&dot1));
assert!(dot2.is_dominating(&dot1));

// You can use the Add and addAssign operators for dots.
let mut dot3 = dot2 + 1;
dot3 += 10;
println!("dot3={:?}", dot3);

// Also you can merge the dot with a new value (assign it only when the value increases)
// using the bitor assign operator
dot3 |= 30;
assert_eq!(30, dot3.counter);
```

For a full example go to the [clock_dot.rs](https://github.com/veminovici/euklid/blob/main/euklid-clocks/examples/clock_dot.rs) file.

<br/>

### Vector Clock
The [VClock](https://github.com/veminovici/euklid/blob/main/src/vclock.rs) is implementing a vector clock.


```rust
use euklid_clocks::*;
use std::iter::FromIterator;

// Create a vclock from a vector of actors.
let mut v1 = VClock::<i32>::from_iter([1, 2, 3]);

// Update the value of for '1' and '3' actors.
v1 |= Dot::new(1, 10);
v1 |= Dot::new(3, 30);

// Create a second vector clock (note that this vector has 4 actors)
let mut v2 = VClock::<i32>::from_iter([1, 2, 3, 4]);
v2 |= Dot::new(1, 15);
v2 |= Dot::new(2, 20);
v2 |= Dot::new(3, 28);

// Merge the two vvector clocks
v1 |= v2;

// Tests that the max values are in place
// Also, the vector clock has values for 4 actors
assert_eq!(15, v1.counter(&1));
assert_eq!(20, v1.counter(&2));
assert_eq!(30, v1.counter(&3));
assert_eq!(0, v1.counter(&4));
```

For a full example go to the [clock_vec.rs](https://github.com/veminovici/euklid/blob/main/euklid-clocks/examples/clock_vec.rs) file.

<br/>

### Dotted-Vector-Value
The crate exposes **Dvv** structure which is an implementation of a dotted-vector-value. For more resources, please check out the resources section.
I found very useful the following two blog posts: [Vector Clocks Revisited](https://riak.com/posts/technical/vector-clocks-revisited/index.html?p=9545.html) and [Vector Clocks Revisited Part2: Dotted Version Vectors](https://riak.com/posts/technical/vector-clocks-revisited-part-2-dotted-version-vectors/index.html).

```rust
use euklid_clocks::*;

// Create a dvv on server '1234'
let mut srv_dvv: Dvv<i32, String> = Dvv::new(1234);

// the client gets the dot from the server
let mut cy_dot = srv_dvv.dot;

// server receives from a client the 'bob' value.
let msg = (cy_dot, "Bob".to_string());

// server merges the received value.
srv_dvv.merge(&msg.0, &msg.1);

// print the ddv content
println!("srv_ddv {:?}", srv_dvv);
```

For a full example go to the [clock_dvv.rs](https://github.com/veminovici/euklid/blob/main/euklid-clocks/examples/clock_dvv.rs) file.

<br/>

### Resources
- [Vector Clocks Revisited](https://riak.com/posts/technical/vector-clocks-revisited/index.html?p=9545.html)
- [Vector Clocks Revisited Part2: Dotted Version Vectors](https://riak.com/posts/technical/vector-clocks-revisited-part-2-dotted-version-vectors/index.html)
- [Version Vector](https://martinfowler.com/articles/patterns-of-distributed-systems/version-vector.html#:~:text=Dotted%20version%20vectors%20One%20of%20the%20major%20problems,time.%20The%20problem%20is%20called%20as%20sibling%20explosion.)
- [A Brief History of Time in Raik](https://speakerdeck.com/seancribbs/a-brief-history-of-time-in-riak)
- [Scalable and Accurate Causality Tracking for Eventual Consistent Stores](https://haslab.uminho.pt/tome/files/dvvset-dais.pdf)
- [A Comprehensive Study of Convergent and Commutative Replicated Data Types](https://hal.inria.fr/file/index/docid/555588/filename/techreport.pdf)
- [John Mumm - A CRDT Primer: Defanging Order Theory](https://www.youtube.com/watch?v=OOlnp2bZVRs)
- [Conflict Free Replicated Data Types on Wiki](https://en.wikipedia.org/wiki/Conflict-free_replicated_data_type)
- [rust-crdt](https://github.com/rust-crdt/rust-crdt)
- [Rust code coverage](https://eipi.xyz/blog/rust-code-coverage-with-github-workflows/)
- [Github workflows for Rust](https://eipi.xyz/blog/github-workflows-to-do-useful-things-with-rust/)

<br/>

### Thank you!!!

> You can contact me at veminovici@hotmail.com. Code designed and written in Päädu, on the beautiful island of [**Saaremaa**](https://goo.gl/maps/DmB9ewY2R3sPGFnTA), Estonia.

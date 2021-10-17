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

// Use some operators (Add and AddAssign) for dots.
let mut dot3 = dot2 + 1;
dot3 += 10;
println!("dot3={:?}", dot3);
```
More examples can be found in the [example](https://github.com/veminovici/euklid/blob/main/examples/dot.rs) file.

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

// Test the counter values for the 3 actors.
assert_eq!(10, v1.counter(&1));
assert_eq!(0, v1.counter(&2));
assert_eq!(30, v1.counter(&3));

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

More examples can be found in the [example](https://github.com/veminovici/euklid/blob/main/examples/vclock.rs) file.

<br/>

### GCounter
The [GCounter](https://github.com/veminovici/euklid/blob/main/src/gcounter.rs) is implementing a grow-only counter.
```rust
extern crate euklid;
use euklid::{Dot, GCounter};

// Create a vclock and increment the counter for user A.
let mut a = VGounter::new();

// Increment the counter for actor A
a.inc("A");

// Increment the counter for actor B
a.inc("B");

// Increase the counter for actor A by 5
a.stepup("A", 5);
```
More examples can be found in the [example](https://github.com/veminovici/euklid/blob/main/examples/gcounter.rs) file.

<br/>

### PNCounter
The [PNCounter](https://github.com/veminovici/euklid/blob/main/src/pncounter.rs) is implementing a pn-counter.
```rust
extern create euklid;
use euklid::{PNCounter};

// Build a new pncounter
let mut a = PNCounter::new();

// Increment the counter value
a.inc("A");

// Increase the counter value by 5.
a.step_up("A", 5));

// Decrement the value
a.decr("A");

// Decrease the counter value by 2
a.step_down("A", 2);

// The counter value should be 3 by now.
assert_eq!(a.counter(), 3);
```
More examples can be found in the [example](https://github.com/veminovici/euklid/blob/main/examples/pncounter.rs) file.

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

### Thank you!!!

> You can contact me at veminovici@hotmail.com. Code designed and written in Päädu, on the beautiful island of [**Saaremaa**](https://goo.gl/maps/DmB9ewY2R3sPGFnTA), Estonia.

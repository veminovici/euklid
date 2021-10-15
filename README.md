# ![rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white) Simplee...Euklid... 

Just another rust crate, this one implements CRDTs things.

[![CI Pipeline](https://github.com/veminovici/euklid/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/veminovici/euklid/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/veminovici/euklid/branch/main/graph/badge.svg?token=1QV7SGC7B7)](https://codecov.io/gh/veminovici/euklid)
[![Coverage Status](https://coveralls.io/repos/github/veminovici/euklid/badge.svg)](https://coveralls.io/github/veminovici/euklid)
[![Tag](https://img.shields.io/github/tag/veminovici/euklid)](https://github.com/veminovici/euklid)
[![Last commit](https://img.shields.io/github/last-commit/veminovici/euklid)](https://github.com/veminovici/euklid)
[![Repo size](https://img.shields.io/github/repo-size/veminovici/euklid)](https://github.com/veminovici/euklid)

[![Github Actions](https://buildstats.info/github/chart/veminovici/euklid)](https://github.com/veminovici/euklid)

<br/>

### Dot
The [Dot](https://github.com/veminovici/euklid/blob/main/src/dot.rs) is implementing a marked version.
```rust
extern crate euklid;
use euklid::Dot;

// Create a dot for actor Alice.
let mut dot = Dot::new("Alice".to_string(), 0);
// Increment the dot.
dot.apply_inc();
```
More examples can be found in the [example](https://github.com/veminovici/euklid/blob/main/examples/dot.rs) file.

<br/>

### Vector Clock
The [VClock](https://github.com/veminovici/euklid/blob/main/src/vclock.rs) is implementing a vector clock.
```rust
extern crate euklid;
use euklid::{Dot, VClock};

// Create a vclock and increment the counter for user A.
let mut a = VClock::new();

// Increment the counter for actor A
a.apply(a.inc_op("A"));

// Increment the counter for actor B
a.apply(a.inc_op("B"));
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
- [A Comprehensive Study of Convergent and Commutative Replicated Data Types](https://hal.inria.fr/file/index/docid/555588/filename/techreport.pdf)
- [John Mumm - A CRDT Primer: Defanging Order Theory](https://www.youtube.com/watch?v=OOlnp2bZVRs)
- [Conflict Free Replicated Data Types on Wiki](https://en.wikipedia.org/wiki/Conflict-free_replicated_data_type)
- [rust-crdt](https://github.com/rust-crdt/rust-crdt)
- [Rust code coverage](https://eipi.xyz/blog/rust-code-coverage-with-github-workflows/)
- [Github workflows for Rust](https://eipi.xyz/blog/github-workflows-to-do-useful-things-with-rust/)

### Thank you!!!

> You can contact me at veminovici@hotmail.com. Code designed and written in Päädu, on the beautiful island of [**Saaremaa**](https://goo.gl/maps/DmB9ewY2R3sPGFnTA), Estonia.

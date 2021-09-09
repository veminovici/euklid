# Simplee |> Euklid
Rust crate for the CRDTs.

[![build](https://github.com/veminovici/euklid/actions/workflows/build.yml/badge.svg)](https://github.com/veminovici/euklid/actions/workflows/build.yml)
[![test](https://github.com/veminovici/euklid/workflows/tests/badge.svg)](https://github.com/veminovici/euklid/actions?query=branch%3Amain+event%3Apush+workflow%3Atests)
[![clippy](https://github.com/veminovici/euklid/workflows/clippy/badge.svg)](https://github.com/veminovici/euklid/actions?query=branch%3Amain+event%3Apush+workflow%3Aclippy)
[![Coverage Status](https://coveralls.io/repos/github/veminovici/euklid/badge.svg?branch=main)](https://coveralls.io/github/veminovici/euklid?branch=main)
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

### G-Counter
The [GCounter](https://github.com/veminovici/euklid/blob/main/src/gcounter.rs) is implementing a grow-only counter.
```rust
xtern crate euklid;
use euklid::{Dot, GCounter};

// Create a vclock and increment the counter for user A.
let mut a = VGounter::new();

// Increment the counter for actor A
a.apply(a.inc_op("A"));

// Increment the counter for actor B
a.apply(a.inc_op("B"));

// Increase the counter for actor A by 5
a.apply(a.step_op("A", 5));
```
More examples can be found in the [example](https://github.com/veminovici/euklid/blob/main/examples/gcounter.rs) file.

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
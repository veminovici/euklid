# Simplee |> Euklid
Rust crate for the CRDTs.

[![Rust](https://github.com/veminovici/euklid/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/veminovici/euklid/actions/workflows/rust.yml)
[![Repo size](https://img.shields.io/github/repo-size/veminovici/euklid)](https://github.com/veminovici/euklid)

[![Github Actions](https://buildstats.info/github/chart/veminovici/euklid)](https://github.com/veminovici/euklid)

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

### Resources
- [A Comprehensive Study of Convergent and Commutative Replicated Data Types](https://hal.inria.fr/file/index/docid/555588/filename/techreport.pdf)
- [John Mumm - A CRDT Primer: Defanging Order Theory](https://www.youtube.com/watch?v=OOlnp2bZVRs)
- [Conflict Free Replicated Data Types on Wiki](https://en.wikipedia.org/wiki/Conflict-free_replicated_data_type)
- [rust-crdt](https://github.com/rust-crdt/rust-crdt)

<br />

### Thank you!!!

> You can contact me at veminovici@hotmail.com. Code designed and written in Päädu, on the beautiful island of [**Saaremaa**](https://goo.gl/maps/DmB9ewY2R3sPGFnTA), Estonia.
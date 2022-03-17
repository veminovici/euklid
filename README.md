# ![rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white) Simplee...Euklid... 

Just another rust crate, this one implements CRDTs things.

[![CI Pipeline](https://github.com/veminovici/euklid/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/veminovici/euklid/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/veminovici/euklid/branch/main/graph/badge.svg?token=IKPMJE7FHB)](https://codecov.io/gh/veminovici/euklid)
[![Coverage Status](https://coveralls.io/repos/github/veminovici/euklid/badge.svg)](https://coveralls.io/github/veminovici/euklid)
[![Tag](https://img.shields.io/github/tag/veminovici/euklid)](https://github.com/veminovici/euklid)
[![Last commit](https://img.shields.io/github/last-commit/veminovici/euklid)](https://github.com/veminovici/euklid)
[![Repo size](https://img.shields.io/github/repo-size/veminovici/euklid)](https://github.com/veminovici/euklid)

## 1. CvRDT, CmRDT, and CRDT
The *CvRDT* trait defines the converging or state based merge synchronization. The *CmRDT* trait defines the commuting or operation based synchronization. The *CRDT* traits defines the access to the inner data value.

```rust
pub trait CvRDT {
    /// Merge the given CRDT into the current CRDT.
    fn merge(&mut self, other: Self);
}

pub trait CmRDT {
    /// Op's must be idempotent, meaning any Op may be applied more than once.
    type Op;

    /// Apply an Op to the CRDT
    fn apply(&mut self, op: Self::Op);
}

pub trait CRDT {
    /// The type of the value.
    type Output;

    /// Returns the value of the CRDT.
    fn value(&self) -> Self::Output;
}
```

## 2. CRDTs
The crate implement different CDRT structure.

### 2.1. GCounter
### 2.2. PNCounter


## 3. Utilities

### 3.1. Causality
The crate defines the **CausalOrdering** enumeration which has 4 values: *equal*, *precede*, *succeed*, *concurrent*. The crate also defines **CausalOrd** trait which along with the **CasualOrdering** allows the caller
to determine if there is any causality between two different instances of the **CausalOrd**.


```rust
pub enum Causality {
    /// An event precedes another event.
    Precede,
    /// An event is equal with another event.
    Equal,
    /// An event succeeds another event.
    Succeed,
    /// An event is concurrent with another event.
    Concurrent,
}

/// A trait that compares two events and returns their causality relation.
pub trait CausalityOrd: PartialOrd {
    /// Returns the causality relation between two entities.
    fn causality_cmp(&self, other: &Self) -> Causality {
        match self.partial_cmp(other) {
            Some(core::cmp::Ordering::Equal) => Causality::Equal,
            Some(core::cmp::Ordering::Less) => Causality::Precede,
            Some(core::cmp::Ordering::Greater) => Causality::Succeed,
            None => Causality::Concurrent,
        }
    }
}
```

### 3.2. Identities, Actors, and Counters Traits

**Identities**: these are traits that define the *zero* and *one* values for a type.

```rust
/// Represents the identity value `zero`.
pub trait Zero {
    /// Returns the `zero` value for the type.
    fn zero() -> Self;
}

/// represents the identity value `one`.
pub trait One {
    /// Returns the `one` value for the type.
    fn one() -> Self;
}
```

**Actors and Counters**: the *Actor* trait defines the expected traits for an actor identifies. The *Counter* trait defines the expected traits for a counter.

```rust
pub trait Actor: Copy + Ord + Zero {}
pub trait Counter: Copy + PartialOrd + Add<Output = Self> + AddAssign + Sub<Output = Self> + SubAssign + Zero + One {}
```

The crate implements the *Actor* and *Counter* traits for all basic numeric types: *usize*, *u8*, *u16*, ..., *i8*, *i16*, ...

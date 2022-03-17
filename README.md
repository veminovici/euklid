# ![rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white) Simplee...Euklid... 

Just another rust crate, this one implements CRDTs things.

[![CI Pipeline](https://github.com/veminovici/euklid/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/veminovici/euklid/actions/workflows/ci.yml)
[![Tag](https://img.shields.io/github/tag/veminovici/euklid)](https://github.com/veminovici/euklid)
[![Last commit](https://img.shields.io/github/last-commit/veminovici/euklid)](https://github.com/veminovici/euklid)
[![Repo size](https://img.shields.io/github/repo-size/veminovici/euklid)](https://github.com/veminovici/euklid)

## 1. Causality

### 1.1. Causality Enumeration
The crate defines the **CausalOrdering** enumeration which has 4 values:

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
```

### 1.2. CausalityOrd Trait
The crate also defines **CausalOrd** trait which along with the **CasualOrdering** allows the caller
to determine if there is any causality between two different instances of the **CausalOrd**.

```rust
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

## 2. Identities, Actors, and Counters Traits

### 2.1. Identities
These are traits that define the *zero* and *one* values for a type.

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
### 2.2. Actors and Counters
The *Actor* trait defines the expected traits for an actor identifies. The *Counter* trait defines the expected traits for a counter.

```rust
pub trait Actor: Copy + Ord + Zero {}
pub trait Counter: Copy + PartialOrd + Add<Output = Self> + AddAssign + Sub<Output = Self> + SubAssign + Zero + One {}
```

The crate implements the *Actor* and *Counter* traits for all basic numeric types: *usize*, *u8*, *u16*, ..., *i8*, *i16*, ...

## 3. CvRDT and CmRDT Traits

### 3.1. CvRDT Trait
The *CvRDT* trait defines the converging or state based merge synchronization.

```rust
pub trait CvRDT {
    /// Merge the given CRDT into the current CRDT.
    fn merge(&mut self, other: Self);
}
```

### 3.2. CmRDT Trait
The *CmRDT* trait defines the commuting or operation based synchronization.

```rust
pub trait CmRDT {
    /// Op's must be idempotent, meaning any Op may be applied more than once.
    type Op;

    /// Apply an Op to the CRDT
    fn apply(&mut self, op: Self::Op);
}
```


## 4. CRDTs

### 4.1. Dot<A: Actor, C: Counter>
### 4.2. VClock
### 4.3. GCounter
### 4.4. PNCounter
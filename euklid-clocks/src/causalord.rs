/// A `CausalOrdering` is the result of causal comparison between two dot values.
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

/// Trait for values that can be compared for a causal order.
///
/// ## How I can implement `CausalOrd`?
///
/// `CausalOrd` only requires implementation of the [`causal_cmp`] method, with the
/// others generated from the default implementations.
///
/// [`causal_cmp`]: CausalOrd::causal_cmp
pub trait CausalOrd: PartialOrd<Self> {
    /// This method returns a causal ordering between `self` and `other` values if one exists.
    fn causal_cmp(&self, other: &Self) -> CausalOrdering {
        match self.partial_cmp(other) {
            Some(std::cmp::Ordering::Equal) => CausalOrdering::Equal,
            Some(std::cmp::Ordering::Less) => CausalOrdering::Precede,
            Some(std::cmp::Ordering::Greater) => CausalOrdering::Succeed,
            None => CausalOrdering::Concurrent,
        }
    }

    /// This method tests succeed (for `self` and `other`)
    fn dominates(&self, other: &Self) -> bool {
        matches!(self.causal_cmp(other), CausalOrdering::Succeed)
    }

    /// This method tests succeed and identical (for `self` and `other`)
    fn descends(&self, other: &Self) -> bool {
        matches!(
            self.causal_cmp(other),
            CausalOrdering::Succeed | CausalOrdering::Equal
        )
    }

    /// This method tests concurrent (for `self` and `other`)
    fn concurrent(&self, other: &Self) -> bool {
        matches!(self.causal_cmp(other), CausalOrdering::Concurrent)
    }
}


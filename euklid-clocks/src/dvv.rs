use crate::{CausalOrd, Dot};
use std::fmt::Debug;

/// Dotted vector value.
pub struct Dvv<A, T> {
    /// The current dot for the value.
    pub dot: Dot<A>,
    values: Vec<(Dot<A>, T)>,
}

impl<A, T> Dvv<A, T> {
    /// Creates a new instance of a dotted-version-value
    pub fn new(actor: A) -> Self {
        Dvv {
            dot: Dot::new(actor, 0),
            values: Vec::new(),
        }
    }
}

impl<A: Debug, T: Debug> std::fmt::Debug for Dvv<A, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dot={:?} vs={:?}", self.dot, self.values)
    }
}

impl<A: Copy + PartialOrd, T: Clone> Dvv<A, T> {
    /// Merges a pair formed from a dot and a value.
    pub fn merge(&mut self, dot: &Dot<A>, v: &T) {
        self.dot.apply_inc_op();
        self.values.retain(|(d, _)| !dot.is_descendant(d));
        self.values.push((self.dot, v.clone()));
    }
}

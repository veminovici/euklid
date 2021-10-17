use crate::{CausalOrd, Dot};
use std::fmt::Debug;
use std::ops::BitOrAssign;

/// Dotted vector value.
///
/// # Example
///
/// ```rust
/// use euklid_clocks::*;
///
/// let mut dvv: Dvv<i32, String> = Dvv::new(1234);
///
/// dvv |= (Dot::new(1234, 0), "Bob".to_string());
/// assert_eq!(1, dvv.dot.counter);
///
/// dvv |= (Dot::new(1234, 0), "Sue".to_string());
/// assert_eq!(2, dvv.dot.counter);
///
/// dvv |= (Dot::new(1234, 1), "Rita".to_string());
/// assert_eq!(3, dvv.dot.counter);
///
/// dvv |= (Dot::new(1234, 2), "Michelle".to_string());
/// assert_eq!(4, dvv.dot.counter);
/// ```
pub struct Dvv<A, T> {
    /// The current dot for the value.
    pub dot: Dot<A>,
    /// The values
    values: Vec<(Dot<A>, T)>,
}

//
// Public functionality
//

impl<A, T> Dvv<A, T> {
    /// Creates a new instance of a dotted-version-value
    pub fn new(actor: A) -> Self {
        Dvv {
            dot: Dot::new(actor, 0),
            values: Vec::new(),
        }
    }
}

impl<A: Copy + PartialOrd, T: Clone> Dvv<A, T> {
    /// Merges a pair formed from a dot and a value.
    pub fn merge(&mut self, dot: &Dot<A>, v: &T) {
        if self.dot.actor == dot.actor {
            self.dot.apply_inc_op();
            self.values.retain(|(d, _)| !dot.is_descendant(d));
            self.values.push((self.dot, v.clone()));
        }
    }
}

//
// Operators
//

impl<A: Copy + PartialOrd, T: Clone> BitOrAssign<(Dot<A>, T)> for Dvv<A, T> {
    fn bitor_assign(&mut self, rhs: (Dot<A>, T)) {
        self.merge(&rhs.0, &rhs.1)
    }
}

//
// Formatting
//

impl<A: Debug, T: Debug> std::fmt::Debug for Dvv<A, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dot={:?} vs={:?}", self.dot, self.values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let dvv: Dvv<i32, u32> = Dvv::new(1);
        assert_eq!(1, dvv.dot.actor);
        assert_eq!(0, dvv.dot.counter);
        assert!(dvv.values.is_empty());
    }

    #[test]
    fn test_debug() {
        let dvv: Dvv<i32, u32> = Dvv::new(1);
        let s = format!("{:?}", dvv);
        assert!(!s.is_empty());
    }

    #[test]
    fn test_merge_diff_actors() {
        let mut dvv1: Dvv<i32, u32> = Dvv::new(1);
        let dot = Dot::new(2, 10);
        dvv1.merge(&dot, &10);

        assert_eq!(1, dvv1.dot.actor);
        assert_eq!(0, dvv1.dot.counter);
        assert!(dvv1.values.is_empty());
    }

    #[test]
    fn test_merge() {
        let mut dvv: Dvv<i32, String> = Dvv::new(1234);
        let dot = Dot::new(1234, 0);
        dvv.merge(&dot, &"Bob".to_string());

        assert_eq!(1, dvv.dot.counter);
        assert_eq!(1, dvv.values.len());

        dvv.merge(&dot, &"Sue".to_string());
        assert_eq!(2, dvv.dot.counter);
        assert_eq!(2, dvv.values.len());

        let dot = Dot::new(1234, 1);
        dvv.merge(&dot, &"Rita".to_string());
        assert_eq!(3, dvv.dot.counter);
        assert_eq!(2, dvv.values.len());

        let dot = Dot::new(1234, 2);
        dvv.merge(&dot, &"Michelle".to_string());
        assert_eq!(4, dvv.dot.counter);
        assert_eq!(2, dvv.values.len());
    }

    #[test]
    fn test_bitor() {
        let mut dvv: Dvv<i32, String> = Dvv::new(1234);

        dvv |= (Dot::new(1234, 0), "Bob".to_string());
        assert_eq!(1, dvv.dot.counter);
        assert_eq!(1, dvv.values.len());

        dvv |= (Dot::new(1234, 0), "Sue".to_string());
        assert_eq!(2, dvv.dot.counter);
        assert_eq!(2, dvv.values.len());

        dvv |= (Dot::new(1234, 1), "Rita".to_string());
        assert_eq!(3, dvv.dot.counter);
        assert_eq!(2, dvv.values.len());

        dvv |= (Dot::new(1234, 2), "Michelle".to_string());
        assert_eq!(4, dvv.dot.counter);
        assert_eq!(2, dvv.values.len());
    }
}

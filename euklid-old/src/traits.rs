/// A state based CRDT.
/// Such CRDTs replicate by transmitting the entire CRDT state.
pub trait CvRDT {
    /// Merge the given CRDT into the current CRDT
    fn merge(&mut self, other: Self);
}

/// An operation based CRDT.
/// Such CRDTs replicate by transmiting each operation.
pub trait CmRDT {
    /// The operation applied to the CRDT
    type Op;

    /// Apply an operation to the CRDT
    fn apply(&mut self, op: Self::Op);
}

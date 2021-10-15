extern crate euklid_old;

use euklid_old::{CmRDT, CvRDT, VClock};

fn main() {
    let mut vclock1 = VClock::new();
    vclock1.apply(vclock1.inc_op("A"));
    eprintln!("vclock a: {:?}", vclock1);
    assert_eq!(vclock1.counter_of(&"A"), 1, "The clock for A must be 1");

    let mut vclock2 = VClock::new();
    vclock2.apply(vclock1.inc_op("B"));
    eprintln!("vclock b: {:?}", vclock2);
    assert_eq!(vclock2.counter_of(&"B"), 1, "The clock for B must be 1");

    vclock1.merge(vclock2);
    eprintln!("vclock merged: {:?}", vclock1);
    vclock1.inc("A");
    assert_eq!(vclock1.counter_of(&"A"), 2, "The clock for A must be 2");
    assert_eq!(vclock1.counter_of(&"B"), 1, "The clock for B must be 1");
}

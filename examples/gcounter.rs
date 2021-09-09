extern crate euklid;

use euklid::{CmRDT, CvRDT, GCounter};

fn main() {
    let mut gcounter1 = GCounter::new();
    gcounter1.inc("A");
    eprintln!("gcounter 1: {:?}", gcounter1);

    let mut gcounter2 = GCounter::new();
    gcounter2.apply(gcounter1.inc_op("B"));
    eprintln!("grounter 2: {:?}", gcounter2);

    gcounter1.merge(gcounter2);
    eprintln!("gcounter merged: {:?}", gcounter1);

    gcounter1.step_up("A", 3);
    gcounter1.inc("B");
    eprintln!("gcounter updated: {:?}", gcounter1);
    eprintln!("counter={}", gcounter1.counter());
}

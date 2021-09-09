extern crate euklid;

use euklid::{CmRDT, CvRDT, PNCounter};

fn main() {
    let mut pncounter1 = PNCounter::new();
    pncounter1.inc("A");
    eprintln!("pncounter 1: {:?}", pncounter1);

    let mut pncounter2 = PNCounter::new();
    pncounter2.apply(pncounter2.inc_op("B"));
    eprintln!("pncounter 2: {:?}", pncounter2);

    pncounter1.merge(pncounter2);
    eprintln!("pncounter merged: {:?}", pncounter1);

    pncounter1.step_up("A", 3);
    pncounter1.inc("B");
    pncounter1.step_down("A", 1);
    eprintln!("counter: {:}", pncounter1.counter());
}

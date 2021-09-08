extern crate euklid;

use euklid::{CmRDT, CvRDT, VClock};

fn main() {
    let mut vclock1 = VClock::new();
    vclock1.apply(vclock1.inc_op("A"));
    eprintln!("vclock a: {:?}", vclock1);

    let mut vclock2 = VClock::new();
    vclock2.apply(vclock1.inc_op("B"));
    eprintln!("vclock b: {:?}", vclock2);

    vclock1.merge(vclock2);
    eprintln!("vclock merged: {:?}", vclock1);
}

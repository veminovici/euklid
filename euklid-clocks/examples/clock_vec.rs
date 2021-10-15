use euklid_clocks::*;
use std::iter::FromIterator;

fn main() {
    let actors = vec![0, 1, 2, 3, 4, 5];
    let vc = VClock::<i32>::from_iter(actors);
    println!("vc={:?}", vc);
    // let xs =
    //     actors
    //     .into_iter()
    //     .map(|a| (a, 0u64))
    //     .collect::<Vec<(i32, u64)>>();

    // let btree = BTreeMap::<i32, u64>::from_iter(xs);
    // println!("btree={:?}", btree);
}

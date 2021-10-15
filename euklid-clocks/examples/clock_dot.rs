use euklid_clocks::*;

fn main() {
    let dot: Dot<i32> = (12345, 1).into();
    println!("dot={:?}", dot);

    let dot1 = dot.inc();
    println!("dot={:?}", dot1);
}

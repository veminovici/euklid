use euklid_clocks::*;

fn main() {
    let dot1: Dot<String> = ("A".to_string(), 1).into();
    println!("dot1={:?}", dot1);

    let dot2 = dot1.incr();
    println!("dot2={:?}", dot2);

    assert!(dot1.descends(&dot1));
    assert!(dot2.descends(&dot1));
    assert!(dot2.dominates(&dot1));
}

use euklid_num::{dot::*, ops::Incrementable};

fn main() {
    let mut c = <i64 as GrowingCounter>::zero();

    c.incr();
    println!("c={}", c);

    c += 10;
    println!("c={}", c);
}

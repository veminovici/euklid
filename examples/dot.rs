extern crate euklid;

use euklid::Dot;

fn main() {
    // Create a dot for the actor Alice.
    let mut dot1 = Dot::new("Alice".to_string(), 0);
    eprintln!("Alice's dot: {:?}", dot1);

    // Increment the dot
    dot1.apply_inc();
    eprintln!("Alice's incremented dot: {:?}", dot1);
    assert_eq!(dot1.counter, 1, "Alice's dot must be 1");

    // Increment one more time the dot
    let dot11 = dot1.inc();
    eprintln!("Alice's incremented dot (again): {:?}", dot11);
    assert_eq!(dot11.counter, 2, "Alice's dot must be 2");

    // Create a second dot.
    let dot2 = Dot::new("Bob".to_string(), 10);
    eprintln!("Bob's dot: {:?}", dot2);
    assert_eq!(dot2.counter, 10, "Bob's dot must be 10");

    // Compare the two dot's
    let e = dot1.eq(&dot2);
    eprintln!("Are the dots comparable? {}", e);
    assert_ne!(e, true, "The two dots cannot be compared");

    // Partial comparation alice's dots
    let e1 = dot11.partial_cmp(&dot1);
    eprintln!("Compare Alice's dots: {:?}", e1);

    // Partial comparation alice and bob
    let e2 = dot11.partial_cmp(&dot2);
    eprintln!("Compare Alice and Bob dots: {:?}", e2);

    let dot3 = Dot::from(("Charlie".to_string(), 0));
    eprintln!("Created dot from pair: {:?}", dot3);
}

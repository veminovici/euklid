use euklid_clocks::*;

// https://riak.com/posts/technical/vector-clocks-revisited-part-2-dotted-version-vectors/index.html

fn main() {
    //
    // INIT
    //

    // Servers '100' starts with an initial dot and an empty DVV.
    let sid = 100;
    let mut srv_dvv: Dvv<i32, String> = Dvv::new(sid);
    println!("S0: srv_ddv {:?}", srv_dvv);

    // client X gets the dot from the server
    let mut cx_dot = srv_dvv.dot;

    // client Y gets the dot from the server
    let mut cy_dot = srv_dvv.dot;

    //
    // CLIENT Y sets value to `bob`
    //

    println!();
    println!("CLIENT Y is sending BOB");

    // client Y sends "bob" to the server.
    let msg = (cy_dot, "Bob".to_string());
    println!("S1: rcv: {:?}", msg);

    // merge the received value
    srv_dvv.merge(&msg.0, &msg.1);
    println!("S2: srv_ddv {:?}", srv_dvv);

    // server sends back to the client Y the value "bob" and the new dot.
    cy_dot = srv_dvv.dot;
    println!("S3: cy: dot={:?}", cy_dot);

    //
    // CLIENT X sets value to `sue`
    //

    println!();
    println!("CLIENT X is sending SUE");

    // client X sends "sue" to the server
    let msg = (cx_dot, "Sue".to_string());
    println!("S4: rcv: {:?}", msg);

    // merge the received value
    srv_dvv.merge(&msg.0, &msg.1);
    println!("S5: srv_ddv {:?}", srv_dvv);

    // server sends back to client x the values ["bob", "sue"] and the new dot.
    cx_dot = srv_dvv.dot;
    println!("S6: cx: dot={:?}", cx_dot);

    //
    // CLIENT Y sets value to 'rita'
    //

    println!();
    println!("CLIENT Y is sending RITA");

    // client Y sends "bob" to the server.
    let msg = (cy_dot, "Rita".to_string());
    println!("S7: rcv: {:?}", msg);

    // merge the received value
    srv_dvv.merge(&msg.0, &msg.1);
    println!("S8: srv_ddv {:?}", srv_dvv);

    // server sends back to the client Y the value "bob" and the new dot.
    cy_dot = srv_dvv.dot;
    println!("S9: cy: dot={:?}", cy_dot);

    //
    // CLIENT X sets value to `sue`
    //

    println!();
    println!("CLIENT X is sending MICHELLE");

    // client X sends "sue" to the server
    let msg = (cx_dot, "Michelle".to_string());
    println!("S10: rcv: {:?}", msg);

    // merge the received value
    srv_dvv.merge(&msg.0, &msg.1);
    println!("S11: srv_ddv {:?}", srv_dvv);

    // server sends back to client x the values ["bob", "sue"] and the new dot.
    cx_dot = srv_dvv.dot;
    println!("S12: cx: dot={:?}", cx_dot);
}

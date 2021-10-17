use euklid_clocks::*;
use std::fmt::Debug;

struct DVValue<A, T> {
    dot: Dot<A>,
    values: Vec<(Dot<A>, T)>,
}

impl<A, T> DVValue<A, T> {
    pub fn new(actor: A) -> Self {
        DVValue {
            dot: Dot::new(actor, 0),
            values: Vec::new(),
        }
    }
}

impl<A: Debug, T: Debug> std::fmt::Debug for DVValue<A, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dot={:?} vs={:?}", self.dot, self.values)
    }
}

impl<A: Copy + PartialOrd, T: Clone> DVValue<A, T> {
    pub fn merge(&mut self, dot: &Dot<A>, v: &T) {
        self.dot.apply_inc_op();
        self.values.retain(|(d, _)| !dot.is_descendant(d));
        self.values.push((self.dot, v.clone()));
    }
}

// https://riak.com/posts/technical/vector-clocks-revisited-part-2-dotted-version-vectors/index.html
fn main() {
    //
    // INIT
    //

    // Servers '100' starts with an initial dot and an empty DVV.
    let sid = 100;
    let mut srv_dvv: DVValue<i32, String> = DVValue::new(sid);
    println!("S0: srv_ddv {:?}", srv_dvv);

    // let mut srv_dot = Dot::new(sid, 0);
    // let mut srv_dvvs: Vec<(Dot<i32>, String)> = Vec::new();
    // println!("T0: srv: dot={:?} ddvs={:?}", srv_dot, srv_dvvs);

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

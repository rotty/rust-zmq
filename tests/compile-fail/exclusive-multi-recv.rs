extern crate zmq;

use std::thread;

macro_rules! t {
    ($e:expr) => (
        $e.unwrap_or_else(|e| { panic!("{} failed with {:?}", stringify!($e), e) })
    )
}

fn main() {
    let mut context = zmq::Context::new();
    let mut socket = t!(context.socket(zmq::REP));
    let i1 = socket.recv_iter(0);
    let i2 = socket.recv_iter(0); //~ ERROR cannot borrow `socket` as mutable more than once at a time [E0499]
}

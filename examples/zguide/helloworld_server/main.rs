#![crate_name = "helloworld_server"]

//! Hello World server in Rust
//! Binds REP socket to tcp://*:5555
//! Expects "Hello" from client, replies with "World"

extern crate zmq;

use std::thread;
use std::time::Duration;

fn main() {
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();

    assert!(responder.bind("tcp://*:5555").is_ok());

    let mut msg = zmq::Message::new().unwrap();
    loop {
        responder.recv(&mut msg, zmq::NOFLAGS).unwrap();
        println!("Received {}", msg.as_str().unwrap());
        thread::sleep(Duration::from_millis(1000));
        responder.send_str("World", zmq::NOFLAGS).unwrap();
    }
}

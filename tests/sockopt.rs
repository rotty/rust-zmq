extern crate zmq;

#[macro_use] mod common;

use zmq::*;
use zmq::sockopt::*;

fn test_getset<T>(sock: Socket, value: <T as OptionSet>::Item)
    where T: OptionSet,
          T: OptionGet,
          <T as OptionGet>::Item: PartialEq<<T as OptionSet>::Item>,
          <T as OptionSet>::Item: std::fmt::Debug,
          <T as OptionGet>::Item: std::fmt::Debug,
          <T as OptionSet>::Item: Clone,
{
    sock.setopt::<T>(value.clone()).unwrap();
    assert_eq!(sock.getopt::<T>().unwrap(), value);
}

test!(test_getset_maxmsgsize, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    test_getset::<MaxMsgSize>(sock, 512000);
});

test!(test_getset_identity, {
    let ctx = Context::new();
    let sock = ctx.socket(REQ).unwrap();
    test_getset::<Identity>(sock, &b"foo"[..]);
});

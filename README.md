Rust ZeroMQ bindings.

[![Travis Build Status](https://travis-ci.org/erickt/rust-zmq.png?branch=master)](https://travis-ci.org/erickt/rust-zmq)
[![Appveyor Build status](https://ci.appveyor.com/api/projects/status/xhytsx4jwyb9qk7m?svg=true)](https://ci.appveyor.com/project/erickt/rust-zmq)
[![Coverage Status](https://coveralls.io/repos/erickt/erickt-zmq/badge.svg?branch=master)](https://coveralls.io/r/erickt/erickt-zmq?branch=master)
[![Apache 2.0 licensed](https://img.shields.io/badge/license-Apache2.0-blue.svg)](./LICENSE-APACHE)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE-MIT)
[![crates.io](http://meritbadge.herokuapp.com/zmq)](https://crates.io/crates/zmq)
[![docs](https://docs.rs/zmq/badge.svg)](https://docs.rs/zmq)

[Documentation](https://docs.rs/crate/zmq/)
[Release Notes](https://github.com/erickt/rust-zmq/tree/master/NEWS.md)

Installation
------------

Currently, rust-zmq requires ZeroMQ 3.2 or newer. For example, on
recent Debian-based distributions, you can use the following command
to get the prerequiste headers and library installed:

    apt install libzmq3-dev

If your OS of choice does not provide packages of a new-enough libzmq,
you will first have to install it from source; see
<https://github.com/zeromq/libzmq/releases>.

rust-zmq uses [cargo](https://crates.io) to install. Users should add this to
their `Cargo.toml` file:

    [dependencies]
    zmq = "0.8"

Install for developers:

    % git clone https://github.com/erickt/rust-zmq
    % cd rust-zmq
    % cargo build

The build normally uses `pkg-config` to find out about libzmq's
location. If that is not available, the environment variable
`LIBZMQ_PREFIX` (or alternatively, `LIBZMQ_LIB_DIR` and
`LIBZMQ_INCLUDE_DIR`) can be defined to avoid the invocation of
`pkg-config`.

Usage
-----

`rust-zmq` is, for the most part, a pretty straight forward port of
the C API into Rust:

```rust
extern crate zmq;

fn main() {
    let ctx = zmq::Context::new();

    let mut socket = ctx.socket(zmq::REQ).unwrap();
    socket.connect("tcp://127.0.0.1:1234").unwrap();
    socket.send_str("hello world!", 0).unwrap();
}
```

You can find more usage examples in
https://github.com/erickt/rust-zmq/tree/master/examples.

Large areas of the API are indeed a direct mapping from C, with the
usual adaptions to Rust language features, such as methods, lifetimes,
richer data types, etc, along with fairly minor tweaks.

In some areas, however, the `rust-zmq` API differs more significantly,
although the mapping should be pretty obvious for each case. Each of
these deviations from the C version of the API are done the improve
the type safety of the Rust binding.

The most significant differences in the 0.8.x series are:

- There are no direct equivalent of `zmq_getsockopt` and
  `zmq_setsockopt`. These are mapped into a pair of getter and setter
  methods for each socket options. While this approach ticks the "type
  safe" box, it sacrifices the ability to abstract over option values
  (issue #134).

- Sockets internally reference, via an `Arc`, the context they were
  created from, which means they will keep their context alive. Thus
  you cannot drop a context to have all sockets indicate an error
  condition on their current or next interaction. You can still
  achieve that effect calling `Context::destroy()`.

extern crate zmq_sys;

use libc::{c_int, size_t, int64_t, uint64_t};
use std::os::raw::c_void;
use std::{mem, ptr, str};
use std::result;

use super::{Result, PollEvents, Socket};

/// Raw 0MQ socket option constants.
const ZMQ_AFFINITY: c_int                 = 4;
const ZMQ_IDENTITY: c_int                 = 5;
const ZMQ_SUBSCRIBE: c_int                = 6;
const ZMQ_UNSUBSCRIBE: c_int              = 7;
const ZMQ_RATE: c_int                     = 8;
const ZMQ_RECOVERY_IVL: c_int             = 9;
const ZMQ_SNDBUF: c_int                   = 11;
const ZMQ_RCVBUF: c_int                   = 12;
const ZMQ_RCVMORE: c_int                  = 13;
const ZMQ_FD: c_int                       = 14;
const ZMQ_EVENTS: c_int                   = 15;
const ZMQ_TYPE: c_int                     = 16;
const ZMQ_LINGER: c_int                   = 17;
const ZMQ_RECONNECT_IVL: c_int            = 18;
const ZMQ_BACKLOG: c_int                  = 19;
const ZMQ_RECONNECT_IVL_MAX: c_int        = 21;
const ZMQ_MAXMSGSIZE: c_int               = 22;
const ZMQ_SNDHWM: c_int                   = 23;
const ZMQ_RCVHWM: c_int                   = 24;
const ZMQ_MULTICAST_HOPS: c_int           = 25;
const ZMQ_RCVTIMEO: c_int                 = 27;
const ZMQ_SNDTIMEO: c_int                 = 28;
const ZMQ_LAST_ENDPOINT: c_int            = 32;
const ZMQ_ROUTER_MANDATORY: c_int         = 33;
const ZMQ_TCP_KEEPALIVE: c_int            = 34;
const ZMQ_TCP_KEEPALIVE_CNT: c_int        = 35;
const ZMQ_TCP_KEEPALIVE_IDLE: c_int       = 36;
const ZMQ_TCP_KEEPALIVE_INTVL: c_int      = 37;
const ZMQ_IMMEDIATE: c_int                = 39;
const ZMQ_XPUB_VERBOSE: c_int             = 40;
const ZMQ_ROUTER_RAW: c_int               = 41;
const ZMQ_IPV6: c_int                     = 42;
const ZMQ_MECHANISM: c_int                = 43;
const ZMQ_PLAIN_SERVER: c_int             = 44;
const ZMQ_PLAIN_USERNAME: c_int           = 45;
const ZMQ_PLAIN_PASSWORD: c_int           = 46;
const ZMQ_CURVE_SERVER: c_int             = 47;
const ZMQ_CURVE_PUBLICKEY: c_int          = 48;
const ZMQ_CURVE_SECRETKEY: c_int          = 49;
const ZMQ_CURVE_SERVERKEY: c_int          = 50;
const ZMQ_PROBE_ROUTER: c_int             = 51;
const ZMQ_REQ_CORRELATE: c_int            = 52;
const ZMQ_REQ_RELAXED: c_int              = 53;
const ZMQ_CONFLATE: c_int                 = 54;
const ZMQ_ZAP_DOMAIN: c_int               = 55;
const ZMQ_ROUTER_HANDOVER: c_int          = 56;
const ZMQ_TOS: c_int                      = 57;
const ZMQ_CONNECT_RID: c_int              = 61;
const ZMQ_GSSAPI_SERVER: c_int            = 62;
const ZMQ_GSSAPI_PRINCIPAL: c_int         = 63;
const ZMQ_GSSAPI_SERVICE_PRINCIPAL: c_int = 64;
const ZMQ_GSSAPI_PLAINTEXT: c_int         = 65;
const ZMQ_HANDSHAKE_IVL: c_int            = 66;
const ZMQ_SOCKS_PROXY: c_int              = 68;
const ZMQ_XPUB_NODROP: c_int              = 69;

pub trait Getter where Self: Sized {
    fn get(sock: *mut c_void, opt: c_int) -> Result<Self>;
}

pub trait Setter where Self: Sized {
    fn set(sock: *mut c_void, opt: c_int, value: Self) -> Result<()>;
}

pub trait Id {
    fn id() -> c_int;
}

pub trait Get: Id {
    type Value;
    fn get(socket: &Socket) -> Result<Self::Value>;
}

pub trait Set<'a>: Id {
    type Value;
    fn set(socket: &Socket, value: Self::Value) -> Result<()>;
}

macro_rules! socket_option(
    ($name:ident, $id:ident, $get_type:ty, $set_type:ty) => (
        pub struct $name;
        impl Id for $name {
            fn id() -> c_int { $id }
        }
        impl Get for $name {
            type Value = $get_type;
            fn get(socket: &Socket) -> Result<$get_type> {
                get::<$get_type>(socket.sock, Self::id())
            }
        }
        impl<'a> Set<'a> for $name {
            type Value = &'a $set_type;
            fn set(socket: &Socket, value: &'a $set_type) -> Result<()> {
                set::<$set_type>(socket.sock, Self::id(), value)
            }
        }
    )
);

macro_rules! socket_option_bytes(
    ($name:ident, $id:ident, $max_len:expr) => (
        pub struct $name;
        impl Id for $name {
            fn id() -> c_int { $id }
        }
        impl Get for $name {
            type Value = Vec<u8>;
            fn get(socket: &Socket) -> Result<Vec<u8>> {
                get_bytes(socket.sock, Self::id(), $max_len)
            }
        }
        impl<'a> Set<'a> for $name {
            type Value = &'a [u8];
            fn set(socket: &Socket, value: &'a [u8]) -> Result<()> {
                set(socket.sock, Self::id(), value)
            }
        }
    )
);

macro_rules! socket_option_string(
    ($name:ident, $id:ident, $max_len:expr) => (
        pub struct $name;
        impl Id for $name {
            fn id() -> c_int { $id }
        }
        impl Get for $name {
            type Value = result::Result<String, Vec<u8>>;
            fn get(socket: &Socket) -> Result<Self::Value> {
                get_string(socket.sock, Self::id(), $max_len, true)
            }
        }
        impl<'a> Set<'a> for $name {
            type Value = &'a str;
            fn set(socket: &Socket, value: &'a str) -> Result<()> {
                set(socket.sock, Self::id(), value)
            }
        }
    )
);

socket_option!(Ipv6, ZMQ_IPV6, bool, bool);
socket_option!(Rcvmore, ZMQ_RCVMORE, bool, bool);
// 255 = longest allowable domain name is 253 so this should be a
// reasonable size.
socket_option_string!(SocksProxy, ZMQ_SOCKS_PROXY, 255);
socket_option_bytes!(Identity, ZMQ_IDENTITY, 255); // 255 = identity max length
socket_option_bytes!(CurvePublicKey, ZMQ_CURVE_PUBLICKEY, 32);

macro_rules! getsockopt_num(
    ($c_ty:ty, $ty:ty) => (
        impl Getter for $ty {
            #[allow(trivial_casts)]
            fn get(sock: *mut c_void, opt: c_int) -> Result<$ty> {
                let mut value: $c_ty = 0;
                let value_ptr = &mut value as *mut $c_ty;
                let mut size = mem::size_of::<$c_ty>() as size_t;

                zmq_try!(unsafe {
                    zmq_sys::zmq_getsockopt(
                        sock,
                        opt,
                        value_ptr as *mut c_void,
                        &mut size)
                });
                Ok(value as $ty)
            }
        }
    )
);

getsockopt_num!(c_int, i32);
getsockopt_num!(int64_t, i64);
getsockopt_num!(uint64_t, u64);

pub fn get_bytes(sock: *mut c_void, opt: c_int, size: size_t) -> Result<Vec<u8>> {
    let mut size = size;
    let mut value = vec![0u8; size];

    zmq_try!(unsafe {
        zmq_sys::zmq_getsockopt(
            sock,
            opt,
            value.as_mut_ptr() as *mut c_void,
            &mut size)
    });
    value.truncate(size);
    Ok(value)
}

pub fn get_string(sock: *mut c_void, opt: c_int, size: size_t, remove_nulbyte: bool)
                  -> Result<result::Result<String, Vec<u8>>> {
    let mut value = try!(get_bytes(sock, opt, size));

    if remove_nulbyte {
        value.pop();
    }
    Ok(String::from_utf8(value).map_err(|e| e.into_bytes()))
}

macro_rules! setsockopt_num(
    ($ty:ty) => (
        impl Setter for $ty {
            #[allow(trivial_casts)]
            fn set(sock: *mut c_void, opt: c_int, value: $ty) -> Result<()> {
                let size = mem::size_of::<$ty>() as size_t;

                zmq_try!(unsafe {
                    zmq_sys::zmq_setsockopt(
                        sock,
                        opt,
                        (&value as *const $ty) as *const c_void,
                        size)
                });
                Ok(())
            }
        }
    )
);

setsockopt_num!(i32);
setsockopt_num!(i64);
setsockopt_num!(u64);

fn setsockopt_null(sock: *mut c_void, opt: c_int) -> Result<()> {
    zmq_try!(unsafe { zmq_sys::zmq_setsockopt(sock, opt, ptr::null(), 0) });
    Ok(())
}

impl<'a> Setter for &'a str {
    fn set(sock: *mut c_void, opt: c_int, value: Self) -> Result<()> {
        set(sock, opt, value.as_bytes())
    }
}

impl<'a> Setter for Option<&'a str> {
    fn set(sock: *mut c_void, opt: c_int, value: Self) -> Result<()> {
        if let Some(s) = value {
            set(sock, opt, s.as_bytes())
        } else {
            setsockopt_null(sock, opt)
        }
    }
}

impl Getter for bool {
    fn get(sock: *mut c_void, opt: c_int) -> Result<Self> {
        let result: i32 = try!(get(sock, opt));
        Ok(result == 1)
    }
}

impl Setter for bool {
    fn set(sock: *mut c_void, opt: c_int, value: Self) -> Result<()> {
        set(sock, opt, if value { 1i32 } else { 0i32 })
    }
}

impl<'a> Setter for &'a [u8] {
    fn set(sock: *mut c_void, opt: c_int, value: &'a [u8]) -> Result<()> {
        zmq_try!(unsafe {
            zmq_sys::zmq_setsockopt(
                sock,
                opt,
                value.as_ptr() as *const c_void,
                value.len() as size_t
            )
        });
        Ok(())
    }
}

impl Getter for PollEvents {
    fn get(sock: *mut c_void, opt: c_int) -> Result<Self> {
        get::<c_int>(sock, opt).map(|bits| PollEvents::from_bits_truncate(bits as i16))
    }
}

pub fn get<T: Getter>(sock: *mut c_void, opt: c_int) -> Result<T> {
    T::get(sock, opt)
}

pub fn set<T: Setter>(sock: *mut c_void, opt: c_int, value: T) -> Result<()> {
    T::set(sock, opt, value)
}

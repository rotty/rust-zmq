extern crate zmq_sys;

use libc::{c_int, c_uint, size_t, int64_t, uint64_t};
use std::os::raw::c_void;
use std::{mem, ptr, str};
use std::result;
use std::marker::PhantomData;

use zmq_sys::RawFd;

use super::{Result, PollEvents};
use super::Constants::*;

pub trait OptionGet {
    type Item;
    fn get(sock: *mut c_void) -> Result<Self::Item>;
}

pub trait OptionSet {
    type Item;
    fn set(sock: *mut c_void, value: Self::Item) -> Result<()>;
}

macro_rules! getset_opt(
    ($name:ident, $ty:ty, $id:expr) => (
        pub struct $name;
        impl OptionGet for $name {
            type Item = $ty;
            fn get(sock: *mut c_void) -> Result<$ty> {
                get::<$ty>(sock, $id.to_raw())
            }
        }
        impl OptionSet for $name {
            type Item = $ty;
            fn set(sock: *mut c_void, value: $ty) -> Result<()> {
                set(sock, $id.to_raw(), value)
            }
        }
    )
);

macro_rules! get_opt(
    ($name:ident, $ty:ty, $id:expr) => (
        pub struct $name;
        impl OptionGet for $name {
            type Item = $ty;
            fn get(sock: *mut c_void) -> Result<$ty> {
                get::<$ty>(sock, $id.to_raw())
            }
        }
    )
);

macro_rules! getset_opt_bytes(
    ($name:ident, $max_len:expr, $id:expr) => (
        pub struct $name<'a>(PhantomData<&'a ()>);

        impl<'a> OptionGet for $name<'a> {
            type Item = Vec<u8>;
            fn get(sock: *mut c_void) -> Result<Vec<u8>> {
                get_bytes(sock, $id.to_raw(), $max_len)
            }
        }

        impl<'a> OptionSet for $name<'a> {
            type Item = &'a [u8];
            fn set(sock: *mut c_void, value: &[u8]) -> Result<()> {
                set(sock, $id.to_raw(), value)
            }
        }
    )
);

macro_rules! set_opt_bytes(
    ($name:ident, $id:expr) => (
        pub struct $name<'a>(PhantomData<&'a ()>);

        impl<'a> OptionSet for $name<'a> {
            type Item = &'a [u8];
            fn set(sock: *mut c_void, value: &[u8]) -> Result<()> {
                set(sock, $id.to_raw(), value)
            }
        }
    )
);

macro_rules! getset_opt_str(
    ($name:ident, $max_len:expr, $id:expr) => (
        pub struct $name<'a>(PhantomData<&'a ()>);

        impl<'a> OptionGet for $name<'a> {
            type Item = result::Result<String, Vec<u8>>;
            fn get(sock: *mut c_void) -> Result<Self::Item> {
                get_string(sock, $id.to_raw(), $max_len, true)
            }
        }

        impl<'a> OptionSet for $name<'a> {
            type Item = &'a str;
            fn set(sock: *mut c_void, value: &str) -> Result<()> {
                set(sock, $id.to_raw(), value)
            }
        }
    )
);

// Some options have special semantics for setting NULL.
macro_rules! getset_opt_str_nullable(
    ($name:ident, $max_len:expr, $id:expr) => (
        pub struct $name<'a>(PhantomData<&'a ()>);

        impl<'a> OptionGet for $name<'a> {
            type Item = result::Result<String, Vec<u8>>;
            fn get(sock: *mut c_void) -> Result<Self::Item> {
                get_string(sock, $id.to_raw(), $max_len, true)
            }
        }

        impl<'a> OptionSet for $name<'a> {
            type Item = Option<&'a str>;
            fn set(sock: *mut c_void, value: Option<&'a str>) -> Result<()> {
                set(sock, $id.to_raw(), value)
            }
        }
    )
);

getset_opt!(MaxMsgSize, i64, ZMQ_MAXMSGSIZE);
getset_opt!(Sndhwm, i32, ZMQ_SNDHWM);
getset_opt!(Rcvhwm, i32, ZMQ_RCVHWM);
getset_opt!(Affinity, u64, ZMQ_AFFINITY);
getset_opt!(Rate, i32, ZMQ_RATE);
getset_opt!(RecoveryIvl, i32, ZMQ_RECOVERY_IVL);
getset_opt!(Sndbuf, i32, ZMQ_SNDBUF);
getset_opt!(Rcvbuf, i32, ZMQ_RCVBUF);
getset_opt!(Tos, i32, ZMQ_TOS);
getset_opt!(Linger, i32, ZMQ_LINGER);
getset_opt!(ReconnectIvl, i32, ZMQ_RECONNECT_IVL);
getset_opt!(ReconnectIvlMax, i32, ZMQ_RECONNECT_IVL_MAX);
getset_opt!(Backlog, i32, ZMQ_BACKLOG);
getset_opt!(Ipv6, bool, ZMQ_IPV6);
getset_opt!(Immediate, bool, ZMQ_IMMEDIATE);
getset_opt!(PlainServer, bool, ZMQ_PLAIN_SERVER);
getset_opt!(Conflate, bool, ZMQ_CONFLATE);
getset_opt!(RouterMandatory, bool, ZMQ_ROUTER_MANDATORY);
getset_opt!(ProbeRouter, bool, ZMQ_PROBE_ROUTER);
getset_opt!(CurveServer, bool, ZMQ_CURVE_SERVER);
getset_opt!(MulticastHops, i32, ZMQ_MULTICAST_HOPS);
getset_opt!(Rcvtimeo, i32, ZMQ_RCVTIMEO);
getset_opt!(Sndtimeo, i32, ZMQ_SNDTIMEO);
getset_opt!(HandshakeIvl, i32, ZMQ_HANDSHAKE_IVL);
getset_opt!(TcpKeepalive, i32, ZMQ_TCP_KEEPALIVE);
getset_opt!(TcpKeepaliveCnt, i32, ZMQ_TCP_KEEPALIVE_CNT);
getset_opt!(TcpKeepaliveIdle, i32, ZMQ_TCP_KEEPALIVE_IDLE);
getset_opt!(TcpKeepaliveIntvl, i32, ZMQ_TCP_KEEPALIVE);
getset_opt_bytes!(Identity, 255, ZMQ_IDENTITY);

get_opt!(Fd, RawFd, ZMQ_FD);
get_opt!(Events, PollEvents, ZMQ_FD);

set_opt_bytes!(Subscribe, ZMQ_SUBSCRIBE);
set_opt_bytes!(Unsubscribe, ZMQ_UNSUBSCRIBE);

// The longest allowable domain name is 253 so 255 should be a
// reasonable size.
getset_opt_str_nullable!(SocksProxy, 255, ZMQ_SOCKS_PROXY);
getset_opt_str!(ZapDomain, 255, ZMQ_ZAP_DOMAIN); // 255 = arbitrary size
getset_opt_str_nullable!(PlainUsername, 255, ZMQ_PLAIN_USERNAME); // 255 = arbitrary size
getset_opt_str_nullable!(PlainPassword, 255, ZMQ_PLAIN_PASSWORD); // 255 = arbitrary size

getset_opt_bytes!(CurvePublickey, 32, ZMQ_CURVE_PUBLICKEY);
getset_opt_bytes!(CurveSecretkey, 32, ZMQ_CURVE_SECRETKEY);
getset_opt_bytes!(CurveServerkey, 32, ZMQ_CURVE_SERVERKEY);

pub trait Getter where Self: Sized {
    fn get(sock: *mut c_void, opt: c_int) -> Result<Self>;
}

pub trait Setter where Self: Sized {
    fn set(sock: *mut c_void, opt: c_int, value: Self) -> Result<()>;
}

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
getsockopt_num!(c_uint, u32);
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

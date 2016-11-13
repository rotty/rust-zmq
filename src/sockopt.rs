extern crate zmq_sys;

use libc::{c_int, size_t, int64_t, uint64_t};
use std::os::raw::c_void;
use std::{mem, ptr, str};
use std::result;

use super::*;

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
                unsafe {
                    let mut value: $c_ty = 0;
                    let value_ptr = &mut value as *mut $c_ty;
                    let mut size = mem::size_of::<$c_ty>() as size_t;

                    let rc = zmq_sys::zmq_getsockopt(
                        sock,
                        opt,
                        value_ptr as *mut c_void,
                        &mut size);

                    if rc == -1 {
                        Err(errno_to_error())
                    } else {
                        Ok(value as $ty)
                    }
                }
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

    let r = unsafe {
        zmq_sys::zmq_getsockopt(
            sock,
            opt,
            value.as_mut_ptr() as *mut c_void,
            &mut size)
    };

    if r == -1i32 {
        Err(errno_to_error())
    } else {
        value.truncate(size);
        Ok(value)
    }
}

pub fn get_string(sock: *mut c_void, opt: c_int, size: size_t, remove_nulbyte: bool) -> Result<result::Result<String, Vec<u8>>> {
    let mut value = try!(get_bytes(sock, opt, size));

    if remove_nulbyte {
        let len = value.len() - 1;
        value.truncate(len);
    }

    Ok(str::from_utf8(&value)
       .map(str::to_string)
       .map_err(|_| value))
}

macro_rules! setsockopt_num(
    ($ty:ty) => (
        impl Setter for $ty {
            #[allow(trivial_casts)]
            fn set(sock: *mut c_void, opt: c_int, value: $ty) -> Result<()> {
                let size = mem::size_of::<$ty>() as size_t;

                let rc = unsafe {
                    zmq_sys::zmq_setsockopt(
                        sock,
                        opt,
                        (&value as *const $ty) as *const c_void,
                        size)
                };

                if rc == -1 {
                    Err(errno_to_error())
                } else {
                    Ok(())
                }
            }
        }
    )
);

setsockopt_num!(i32);
setsockopt_num!(i64);
setsockopt_num!(u64);

fn setsockopt_null(sock: *mut c_void, opt: c_int) -> Result<()> {
    let r = unsafe {
        zmq_sys::zmq_setsockopt(
            sock,
            opt,
            ptr::null(),
            0
        )
    };

    if r == -1i32 {
        Err(errno_to_error())
    } else {
        Ok(())
    }
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
        let r = unsafe {
            zmq_sys::zmq_setsockopt(
                sock,
                opt,
                value.as_ptr() as *const c_void,
                value.len() as size_t
            )
        };

        if r == -1i32 {
            Err(errno_to_error())
        } else {
            Ok(())
        }
    }
}

pub fn get<T: Getter>(sock: *mut c_void, opt: c_int) -> Result<T> {
    T::get(sock, opt)
}

pub fn set<T: Setter>(sock: *mut c_void, opt: c_int, value: T) -> Result<()> {
    T::set(sock, opt, value)
}

// FIXME: duplicated from lib.rs
fn errno_to_error() -> Error {
    Error::from_raw(unsafe { zmq_sys::zmq_errno() })
}

//#![crate_name = "libovr"]
#![crate_type = "lib"]
//#![feature(link_args)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

extern crate libc;

use libc::{c_int, c_uint, c_void, c_float, c_double};
use std::default::Default;
use std::ptr;
use std::string::raw::from_buf;

mod link_settings;

pub mod ffi;



#![allow(dead_code, non_camel_case_types, non_snake_case)]

extern crate libc;
extern crate xcb;

use libc::*;
use xcb::ffi::*;

include!("vulkan.rs");

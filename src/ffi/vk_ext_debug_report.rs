#![allow(dead_code, non_camel_case_types, non_snake_case)]

use libc::*;
use ffi::vulkan::*;

pub const VK_EXT_DEBUG_REPORT_EXTENSION_NAME: &'static str = "VK_EXT_debug_report";

include!("gen/vk_ext_debug_report.rs");

#![cfg(target_os = "linux")]
#![cfg(feature = "xlib_surface")]

use vk::*;

pub const SPEC_VERSION: u32 = 6;
pub const EXTENSION_NAME: &'static str = "VK_KHR_xlib_surface";

impl_enum!{StructureType;
    XLIB_SURFACE_CREATE_INFO_KHR = 1000004000,
}

#![cfg(target_os = "android")]

use vk::*;

pub const SPEC_VERSION: u32 = 6;
pub const EXTENSION_NAME: &'static str = "VK_KHR_android_surface";

impl_enum!{StructureType;
    ANDROID_SURFACE_CREATE_INFO = 1000008000,
}

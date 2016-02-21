#![cfg(target_os = "windows")]
pub const SPEC_VERSION: u32 = 5;
pub const EXTENSION_NAME: &'static str = "VK_KHR_win32_surface";

impl_enum!{StructureType;
    WIN32_SURFACE_CREATE_INFO_KHR = 1000009000,
}

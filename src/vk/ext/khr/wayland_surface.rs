#![cfg(feature = "wayland")]
pub const SPEC_VERSION: u32 = 5;
pub const EXTENSION_NAME: &'static str = "VK_KHR_wayland_surface";

impl_enum!{StructureType;
    WAYLAND_SURFACE_CREATE_INFO_KHR = 1000006000,
}

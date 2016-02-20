#![cfg(feature = "xcb")]
pub const SPEC_VERSION: u32 = 6;
pub const EXTENSION_NAME: &'static str = "VK_KHR_xcb_surface";

impl_enum!{StructureType;
    XCB_SURFACE_CREATE_INFO_KHR = 1000005000,
}

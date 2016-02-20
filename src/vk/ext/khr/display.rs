use vk::*;

pub const SPEC_VERSION: u32 = 21;
pub const EXTENSION_NAME: &'static str = "VK_KHR_display";

impl_enum!{StructureType;
    DISPLAY_MODE_CREATE_INFO_KHR = 1000002000,
    DISPLAY_SURFACE_CREATE_INFO_KHR = 1000002001,
}

use vk::*;

pub const SPEC_VERSION: u32 = 25;
pub const EXTENSION_NAME: &'static str = "VK_KHR_surface";

impl_enum!{Result;
    ERROR_SURFACE_LOST_KHR = -1000000000,
    ERROR_NATIVE_WINDOW_IN_USE_KHR = -1000000001,
}

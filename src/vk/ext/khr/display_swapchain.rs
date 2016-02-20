use vk::*;

pub const SPEC_VERSION: u32 = 9;
pub const EXTENSION_NAME: &'static str = "VK_KHR_display_swapchain";

impl_enum!{Result;
    ERROR_INCOMPATIBLE_DISPLAY_KHR = -1000003001,
}

impl_enum!{StructureType;
    DISPLAY_PRESENT_INFO_KHR = 1000003000,
}

use vk::*;

pub const SPEC_VERSION: u32 = 67;
pub const EXTENSION_NAME: &'static str = "VK_KHR_swapchain";

impl_enum!{Result;
    SUBOPTIMAL = 1000001003,
    ERROR_OUT_OF_DATE = -1000001004,
}
impl_enum!{StructureType;
    SWAPCHAIN_CREATE_INFO = 1000001000,
    PRESENT_INFO = 1000001001,
}
impl_enum!{ImageLayout;
    PRESENT_SRC = 1000001002,
}

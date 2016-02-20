pub const SPEC_VERSION: u32 = 67;
pub const EXTENSION_NAME: &'static str = "VK_KHR_swapchain";

impl_enum!{Result;
    SUBOPTIMAL_KHR = 1000001003,
    ERROR_OUT_OF_DATE_KHR = -1000001004,
}

impl_enum!{StructureType;
    SWAPCHAIN_CREATE_INFO_KHR = 1000001000,
    PRESENT_INFO_KHR = 1000001001,
}

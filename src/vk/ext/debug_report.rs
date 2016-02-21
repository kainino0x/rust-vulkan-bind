use vk::*;

pub const SPEC_VERSION: u32 = 2;
pub const EXTENSION_NAME: &'static str = "VK_EXT_debug_report";

impl_enum!{Result;
    ERROR_VALIDATION_FAILED = -1000011001,
}
impl_enum!{StructureType;
    STRUCTURE_TYPE_DEBUG_REPORT_CREATE_INFO_EXT = 1000011000,
}

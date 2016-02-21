use vk::*;

pub const SPEC_VERSION: u32 = 21;
pub const EXTENSION_NAME: &'static str = "VK_KHR_display";

impl_enum!{StructureType;
    DISPLAY_MODE_CREATE_INFO = 1000002000,
    DISPLAY_SURFACE_CREATE_INFO = 1000002001,
}
make_flag!{DisplayPlaneAlphaFlags;
    OPAQUE = 0x00000001,
    GLOBAL = 0x00000002,
    PER_PIXEL = 0x00000004,
    PER_PIXEL_PREMULTIPLIED = 0x00000008,
}

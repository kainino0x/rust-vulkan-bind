use vk::*;

pub const SPEC_VERSION: u32 = 2;
pub const EXTENSION_NAME: &'static str = "VK_debug_report";

impl_enum!{Result;
    ERROR_VALIDATION_FAILED = -1000011001,
}
impl_enum!{StructureType;
    STRUCTURE_TYPE_DEBUG_REPORT_CREATE_INFO = 1000011000,
}
make_enum!{ObjectType;
    UNKNOWN = 0,
    INSTANCE = 1,
    PHYSICAL_DEVICE = 2,
    DEVICE = 3,
    QUEUE = 4,
    SEMAPHORE = 5,
    COMMAND_BUFFER = 6,
    FENCE = 7,
    DEVICE_MEMORY = 8,
    BUFFER = 9,
    IMAGE = 10,
    EVENT = 11,
    QUERY_POOL = 12,
    BUFFER_VIEW = 13,
    IMAGE_VIEW = 14,
    SHADER_MODULE = 15,
    PIPELINE_CACHE = 16,
    PIPELINE_LAYOUT = 17,
    RENDER_PASS = 18,
    PIPELINE = 19,
    DESCRIPTOR_SET_LAYOUT = 20,
    SAMPLER = 21,
    DESCRIPTOR_POOL = 22,
    DESCRIPTOR_SET = 23,
    FRAMEBUFFER = 24,
    COMMAND_POOL = 25,
    SURFACE_KHR = 26,
    SWAPCHAIN_KHR = 27,
    DEBUG_REPORT = 28,
}
make_enum!{Error;
    NONE = 0,
    CALLBACK_REF = 1,
}
make_flag!{Flags;
    INFO = 0x00000001,
    WARN = 0x00000002,
    PERF_WARN = 0x00000004,
    ERROR = 0x00000008,
    DEBUG = 0x00000010,
}

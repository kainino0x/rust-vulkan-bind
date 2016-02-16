use std;

pub const API_VERSION:                   u32 = (1 << 22) | (0 << 12) | (2);

pub const LOD_CLAMP_NONE:                f32 = 1000.0;
pub const REMAINING_MIP_LEVELS:          u32 = std::u32::MAX;
pub const REMAINING_ARRAY_LAYERS:        u32 = std::u32::MAX;
pub const WHOLE_SIZE:                    u64 = std::u64::MAX;
pub const ATTACHMENT_UNUSED:             u32 = std::u32::MAX;
pub const QUEUE_FAMILY_IGNORED:          u32 = std::u32::MAX;
pub const SUBPASS_EXTERNAL:              u32 = std::u32::MAX;
pub const MAX_PHYSICAL_DEVICE_NAME_SIZE: u32 = 256;
pub const UUID_SIZE:                     u32 = 16;
pub const MAX_MEMORY_TYPES:              u32 = 32;
pub const MAX_MEMORY_HEAPS:              u32 = 16;
pub const MAX_EXTENSION_NAME_SIZE:       u32 = 256;
pub const MAX_DESCRIPTION_SIZE:          u32 = 256;

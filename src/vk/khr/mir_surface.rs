#![cfg(target_os = "linux")]
#![cfg(feature = "mir_surface")]

pub const SPEC_VERISON: u32 = 4;
pub const EXTENSION_NAME: &'static str = "VK_KHR_mir_surface";

impl_enum!{StructureType;
    MIR_SURFACE_CREATE_INFO = 1000007000,
}

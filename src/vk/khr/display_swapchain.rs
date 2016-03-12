use vk::*;
use libc::*;

pub const SPEC_VERSION: u32 = 9;
pub const EXTENSION_NAME: &'static str = "VK_KHR_display_swapchain";

impl_enum!{Result;
    ERROR_INCOMPATIBLE_DISPLAY = -1000003001,
}

impl_enum!{StructureType;
    DISPLAY_PRESENT_INFO = 1000003000,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayPresentInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub srcRect: Rect2D,
    pub dstRect: Rect2D,
    pub persistent: Bool32,
}

pub type PFN_vkCreateSharedSwapchainsKHR =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               swapchainCount: u32,
                                               pCreateInfos: *const khr::swapchain::CreateInfo,
                                               pAllocator: *const AllocationCallbacks,
                                               pSwapchains: *mut khr::swapchain::Swapchain)
                              -> Result>;

#[link(name = "vulkan")]
extern "C" {
    pub fn vkCreateSharedSwapchains(device: Device,
                                    swapchainCount: uint32_t,
                                    pCreateInfos: *const khr::swapchain::CreateInfo,
                                    pAllocator: *const AllocationCallbacks,
                                    pSwapchains: *mut khr::swapchain::Swapchain) -> Result;
}

use vk::*;
use libc::*;

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
make_flag!{CreateFlag; CreateFlags; }

opaque!{_Swapchain, Swapchain}

#[repr(C)]
pub struct CreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: CreateFlags,
    pub surface: khr::surface::Surface,
    pub minImageCount: u32,
    pub imageFormat: Format,
    pub imageColorSpace: khr::surface::ColorSpace,
    pub imageExtent: Extent2D,
    pub imageArrayLayers: u32,
    pub imageUsage: ImageUsageFlags,
    pub imageSharingMode: SharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
    pub preTransform: khr::surface::TransformFlag,
    pub compositeAlpha: khr::surface::CompositeAlphaFlag,
    pub presentMode: khr::surface::PresentMode,
    pub clipped: Bool32,
    pub oldSwapchain: khr::swapchain::Swapchain,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PresentInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphores: *const Semaphore,
    pub swapchainCount: u32,
    pub pSwapchains: *const Swapchain,
    pub pImageIndices: *const u32,
    pub pResults: *mut Result,
}

pub type PFN_vkCreateSwapchain =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               pCreateInfo: *const CreateInfo,
                                               pAllocator: *const AllocationCallbacks,
                                               pSwapchain: *mut Swapchain)
                              -> Result>;
pub type PFN_vkDestroySwapchain =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               swapchain: Swapchain,
                                               pAllocator: *const AllocationCallbacks)>;
pub type PFN_vkGetSwapchainImages =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               swapchain: Swapchain,
                                               pSwapchainImageCount: *mut u32,
                                               pSwapchainImages: *mut Image)
                              -> Result>;
pub type PFN_vkAcquireNextImage =
    ::std::option::Option<unsafe extern "C" fn(device: Device,
                                               swapchain: Swapchain,
                                               timeout: u64,
                                               semaphore: Semaphore,
                                               fence: Fence,
                                               pImageIndex: *mut u32)
                              -> Result>;
pub type PFN_vkQueuePresent =
    ::std::option::Option<unsafe extern "C" fn(queue: Queue,
                                               pPresentInfo: *const PresentInfo)
                              -> Result>;

#[link(name = "vulkan")]
extern "C" {
    pub fn vkCreateSwapchain(device: Device,
                             pCreateInfo: *const khr::swapchain::CreateInfo,
                             pAllocator: *const AllocationCallbacks,
                             pSwapchain: *mut khr::swapchain::Swapchain) -> Result;
    pub fn vkDestroySwapchain(device: Device, swapchain: khr::swapchain::Swapchain,
                              pAllocator: *const AllocationCallbacks);
    pub fn vkGetSwapchainImages(device: Device,
                                swapchain: khr::swapchain::Swapchain,
                                pSwapchainImageCount: *mut uint32_t,
                                pSwapchainImages: *mut Image) -> Result;
    pub fn vkAcquireNextImage(device: Device, swapchain: khr::Swapchain,
                              timeout: uint64_t, semaphore: Semaphore,
                              fence: Fence, pImageIndex: *mut uint32_t) -> Result;
    pub fn vkQueuePresent(queue: Queue,
                          pPresentInfo: *const khr::swapchain::PresentInfo) -> Result;
}

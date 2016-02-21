#![cfg(target_os = "linux")]
#![cfg(feature = "xcb_surface")]

extern crate xcb;

use vk::*;
use self::xcb::ffi::*;

pub const SPEC_VERSION: u32 = 6;
pub const EXTENSION_NAME: &'static str = "VK_KHR_xcb_surface";

impl_enum!{StructureType;
    XCB_SURFACE_CREATE_INFO = 1000005000,
}
make_flag!{CreateFlag; CreateFlags; }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: CreateFlags,
    pub connection: *mut xcb_connection_t,
    pub window: xcb_window_t,
}

#[link(name = "vulkan")]
extern "C" {
    pub fn vkCreateXcbSurfaceKHR(instance: Instance,
                                 pCreateInfo: *const khr::xcb_surface::CreateInfo,
                                 pAllocator: *const AllocationCallbacks,
                                 pSurface: *mut khr::surface::Surface) -> Result;
    pub fn vkGetPhysicalDeviceXcbPresentationSupportKHR(physicalDevice: PhysicalDevice,
                                                        queueFamilyIndex: u32,
                                                        connection: *mut xcb_connection_t,
                                                        visual_id: xcb_visualid_t) -> Bool32;
}

pub type PFN_vkCreateXcbSurfaceKHR =
    ::std::option::Option<unsafe extern "C" fn(instance: Instance,
                                               pCreateInfo: *const CreateInfo,
                                               pAllocator: *const AllocationCallbacks,
                                               pSurface: *mut khr::surface::Surface)
                              -> Result>;
pub type PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR =
    ::std::option::Option<unsafe extern "C" fn(physicalDevice: PhysicalDevice,
                                               queueFamilyIndex: u32,
                                               connection: *mut xcb_connection_t,
                                               visual_id: xcb_visualid_t)
                              -> Bool32>;

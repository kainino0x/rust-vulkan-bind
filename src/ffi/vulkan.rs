#![allow(dead_code, non_camel_case_types, non_snake_case)]

use libc::*;
use xcb::ffi::*;

pub const VK_API_VERSION: u32 = (1 << 22) | (0 << 12) | (2);
//#define VK_LOD_CLAMP_NONE                 1000.0f
//#define VK_REMAINING_MIP_LEVELS           (~0U)
//#define VK_REMAINING_ARRAY_LAYERS         (~0U)
//#define VK_WHOLE_SIZE                     (~0ULL)
//#define VK_ATTACHMENT_UNUSED              (~0U)
//#define VK_TRUE                           1
//#define VK_FALSE                          0
//#define VK_QUEUE_FAMILY_IGNORED           (~0U)
//#define VK_SUBPASS_EXTERNAL               (~0U)
//#define VK_MAX_PHYSICAL_DEVICE_NAME_SIZE  256
//#define VK_UUID_SIZE                      16
//#define VK_MAX_MEMORY_TYPES               32
//#define VK_MAX_MEMORY_HEAPS               16
//#define VK_MAX_EXTENSION_NAME_SIZE        256
//#define VK_MAX_DESCRIPTION_SIZE           256
//#define VK_KHR_SURFACE_SPEC_VERSION       25
//#define VK_KHR_SURFACE_EXTENSION_NAME     "VK_KHR_surface"
pub const VK_KHR_SURFACE_EXTENSION_NAME: &'static str = "VK_KHR_surface";
//#define VK_KHR_SWAPCHAIN_SPEC_VERSION     67
//#define VK_KHR_SWAPCHAIN_EXTENSION_NAME   "VK_KHR_swapchain"
//#define VK_KHR_DISPLAY_SPEC_VERSION       21
//#define VK_KHR_DISPLAY_EXTENSION_NAME     "VK_KHR_display"
//#define VK_KHR_DISPLAY_SWAPCHAIN_SPEC_VERSION 9
//#define VK_KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME "VK_KHR_display_swapchain"
//#define VK_KHR_XLIB_SURFACE_SPEC_VERSION  6
//#define VK_KHR_XLIB_SURFACE_EXTENSION_NAME "VK_KHR_xlib_surface"
//#define VK_KHR_XCB_SURFACE_SPEC_VERSION   6
//#define VK_KHR_XCB_SURFACE_EXTENSION_NAME "VK_KHR_xcb_surface"
pub const VK_KHR_XCB_SURFACE_EXTENSION_NAME: &'static str = "VK_KHR_xcb_surface";
//#define VK_KHR_WAYLAND_SURFACE_SPEC_VERSION 5
//#define VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME "VK_KHR_wayland_surface"
//#define VK_KHR_MIR_SURFACE_SPEC_VERSION   4
//#define VK_KHR_MIR_SURFACE_EXTENSION_NAME "VK_KHR_mir_surface"
//#define VK_KHR_ANDROID_SURFACE_SPEC_VERSION 6
//#define VK_KHR_ANDROID_SURFACE_EXTENSION_NAME "VK_KHR_android_surface"
//#define VK_KHR_WIN32_SURFACE_SPEC_VERSION 5
//#define VK_KHR_WIN32_SURFACE_EXTENSION_NAME "VK_KHR_win32_surface"

include!("gen/vulkan.rs");

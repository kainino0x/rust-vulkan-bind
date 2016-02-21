use vk::*;

pub const SPEC_VERSION: u32 = 21;
pub const EXTENSION_NAME: &'static str = "VK_KHR_display";

impl_enum!{StructureType;
    DISPLAY_MODE_CREATE_INFO = 1000002000,
    DISPLAY_SURFACE_CREATE_INFO = 1000002001,
}
make_flag!{PlaneAlphaFlag; PlaneAlphaFlags;
    OPAQUE = 0x00000001,
    GLOBAL = 0x00000002,
    PER_PIXEL = 0x00000004,
    PER_PIXEL_PREMULTIPLIED = 0x00000008,
}
make_flag!{ModeCreateFlag; ModeCreateFlags; }
make_flag!{CreateFlag; CreateFlags; }

pub enum Display { }
pub enum Mode { }

#[repr(C)]
pub struct Properties {
    pub display: Display,
    pub displayName: *const ::std::os::raw::c_char,
    pub physicalDimensions: Extent2D,
    pub physicalResolution: Extent2D,
    pub supportedTransforms: khr::surface::TransformFlags,
    pub planeReorderPossible: Bool32,
    pub persistentContent: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ModeParameters {
    pub visibleRegion: Extent2D,
    pub refreshRate: u32,
}
#[repr(C)]
pub struct ModeProperties {
    pub displayMode: Mode,
    pub parameters: ModeParameters,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ModeCreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: ModeCreateFlags,
    pub parameters: ModeParameters,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PlaneCapabilities {
    pub supportedAlpha: PlaneAlphaFlags,
    pub minSrcPosition: Offset2D,
    pub maxSrcPosition: Offset2D,
    pub minSrcExtent: Extent2D,
    pub maxSrcExtent: Extent2D,
    pub minDstPosition: Offset2D,
    pub maxDstPosition: Offset2D,
    pub minDstExtent: Extent2D,
    pub maxDstExtent: Extent2D,
}
#[repr(C)]
pub struct PlaneProperties {
    pub currentDisplay: Display,
    pub currentStackIndex: u32,
}
#[repr(C)]
pub struct SurfaceCreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: CreateFlags,
    pub displayMode: Mode,
    pub planeIndex: u32,
    pub planeStackIndex: u32,
    pub transform: khr::surface::TransformFlag,
    pub globalAlpha: ::std::os::raw::c_float,
    pub alphaMode: PlaneAlphaFlag,
    pub imageExtent: Extent2D,
}

#[link(name = "vulkan")]
extern "C" {
    pub fn vkGetPhysicalDeviceDisplayPropertiesKHR(physicalDevice: PhysicalDevice,
                                                   pPropertyCount: *mut u32,
                                                   pProperties: *mut khr::display::Properties) -> Result;
    pub fn vkGetPhysicalDeviceDisplayPlanePropertiesKHR(physicalDevice: PhysicalDevice,
                                                        pPropertyCount: *mut u32,
                                                        pProperties: *mut khr::display::PlaneProperties) -> Result;
    pub fn vkGetDisplayPlaneSupportedDisplaysKHR(physicalDevice: PhysicalDevice,
                                                 planeIndex: u32,
                                                 pDisplayCount: *mut u32,
                                                 pDisplays: *mut khr::display::Display) -> Result;
    pub fn vkGetDisplayModePropertiesKHR(physicalDevice: PhysicalDevice,
                                         display: khr::display::Display,
                                         pPropertyCount: *mut u32,
                                         pProperties: *mut khr::display::ModeProperties) -> Result;
    pub fn vkCreateDisplayModeKHR(physicalDevice: PhysicalDevice,
                                  display: khr::display::Display,
                                  pCreateInfo: *const khr::display::ModeCreateInfo,
                                  pAllocator: *const AllocationCallbacks,
                                  pMode: *mut khr::display::Mode) -> Result;
    pub fn vkGetDisplayPlaneCapabilitiesKHR(physicalDevice: PhysicalDevice,
                                            mode: khr::display::Mode,
                                            planeIndex: u32,
                                            pCapabilities: *mut khr::display::PlaneCapabilities) -> Result;
    pub fn vkCreateDisplayPlaneSurfaceKHR(instance: Instance,
                                          pCreateInfo: *const khr::display::SurfaceCreateInfo,
                                          pAllocator: *const AllocationCallbacks,
                                          pSurface: *mut khr::surface::Surface) -> Result;
}

pub type PFN_vkGetPhysicalDeviceDisplayPropertiesKHR =
    ::std::option::Option<unsafe extern "C" fn(physicalDevice: PhysicalDevice,
                                               pPropertyCount: *mut u32,
                                               pProperties: *mut Properties)
                              -> Result>;
pub type PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR =
    ::std::option::Option<unsafe extern "C" fn(physicalDevice: PhysicalDevice,
                                               pPropertyCount: *mut u32,
                                               pProperties: *mut PlaneProperties)
                              -> Result>;
pub type PFN_vkGetDisplayPlaneSupportedDisplaysKHR =
    ::std::option::Option<unsafe extern "C" fn(physicalDevice: PhysicalDevice,
                                               planeIndex: u32,
                                               pDisplayCount: *mut u32,
                                               pDisplays: *mut Display)
                              -> Result>;
pub type PFN_vkGetDisplayModePropertiesKHR =
    ::std::option::Option<unsafe extern "C" fn(physicalDevice: PhysicalDevice,
                                               display: Display,
                                               pPropertyCount: *mut u32,
                                               pProperties: *mut ModeProperties)
                              -> Result>;
pub type PFN_vkCreateDisplayModeKHR =
    ::std::option::Option<unsafe extern "C" fn(physicalDevice: PhysicalDevice,
                                               display: Display,
                                               pCreateInfo: *const ModeCreateInfo,
                                               pAllocator: *const AllocationCallbacks,
                                               pMode: *mut Mode)
                              -> Result>;
pub type PFN_vkGetDisplayPlaneCapabilitiesKHR =
    ::std::option::Option<unsafe extern "C" fn(physicalDevice: PhysicalDevice,
                                               mode: Mode,
                                               planeIndex: u32,
                                               pCapabilities: *mut PlaneCapabilities)
                              -> Result>;
pub type PFN_vkCreateDisplayPlaneSurfaceKHR =
    ::std::option::Option<unsafe extern "C" fn(instance: Instance,
                                               pCreateInfo: *const SurfaceCreateInfo,
                                               pAllocator: *const AllocationCallbacks,
                                               pSurface: *mut khr::surface::Surface)
                              -> Result>;

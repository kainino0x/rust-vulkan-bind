use std;
use std::mem;
use std::ptr::{null, null_mut};

use xcb::ffi::base::*;
use xcb::ffi::xproto::*;
use vulkan_bind::vk;

use util::*;

#[derive(Debug, Clone)]
pub struct SwapchainBuffer {
    image: vk::Image,
    view: vk::ImageView,
}
impl Default for SwapchainBuffer {
    fn default() -> Self {
        SwapchainBuffer {
            image: null_mut(),
            view: null_mut(),
        }
    }
}

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Swapchain {
    instance: vk::Instance,
    device: vk::Device,
    physical_device: vk::PhysicalDevice,
    surface: vk::khr::Surface,
    // Function pointers
    fpGetPhysicalDeviceSurfaceSupport:      vk::khr::surface::PFN_vkGetPhysicalDeviceSurfaceSupport,
    fpGetPhysicalDeviceSurfaceCapabilities: vk::khr::surface::PFN_vkGetPhysicalDeviceSurfaceCapabilities,
    fpGetPhysicalDeviceSurfaceFormats:      vk::khr::surface::PFN_vkGetPhysicalDeviceSurfaceFormats,
    fpGetPhysicalDeviceSurfacePresentModes: vk::khr::surface::PFN_vkGetPhysicalDeviceSurfacePresentModes,
    fpCreateSwapchain:                      vk::khr::swapchain::PFN_vkCreateSwapchain,
    fpDestroySwapchain:                     vk::khr::swapchain::PFN_vkDestroySwapchain,
    fpGetSwapchainImages:                   vk::khr::swapchain::PFN_vkGetSwapchainImages,
    fpAcquireNextImage:                     vk::khr::swapchain::PFN_vkAcquireNextImage,
    fpQueuePresent:                         vk::khr::swapchain::PFN_vkQueuePresent,
    pub color_format: vk::Format,
    pub color_space: vk::khr::surface::ColorSpace,
    pub swapchain: Option<vk::khr::Swapchain>,
    pub image_count: u32,
    pub images: Vec<vk::Image>,
    pub buffers: Vec<SwapchainBuffer>,
    /// Index of the detected graphics and presenting device queue
    pub queue_node_index: u32,
}

macro_rules! get_instance_proc_addr {
    ($inst:expr, $entrypoint:expr) => {{
        let f = vk::vkGetInstanceProcAddr($inst, $entrypoint.as_ptr() as *const i8);
        mem::transmute(f)
    }}
}

macro_rules! get_device_proc_addr {
    ($dev:expr, $entrypoint:expr) => {{
        let f = vk::vkGetDeviceProcAddr($dev, $entrypoint.as_ptr() as *const i8);
        mem::transmute(f)
    }}
}

impl Swapchain {
    #[allow(non_snake_case)]
    pub fn new(instance: vk::Instance,
               physical_device: vk::PhysicalDevice,
               device: vk::Device,
               connection: *mut xcb_connection_t,
               window: xcb_window_t) -> Self {
        // ## connect
        let                                            fpGetPhysicalDeviceSurfaceSupport:
                                 vk::khr::surface::PFN_vkGetPhysicalDeviceSurfaceSupport =
            unsafe { get_instance_proc_addr!(instance, "vkGetPhysicalDeviceSurfaceSupport") };
        let                                            fpGetPhysicalDeviceSurfaceCapabilities:
                                 vk::khr::surface::PFN_vkGetPhysicalDeviceSurfaceCapabilities =
            unsafe { get_instance_proc_addr!(instance, "vkGetPhysicalDeviceSurfaceCapabilities") };
        let                                            fpGetPhysicalDeviceSurfaceFormats:
                                 vk::khr::surface::PFN_vkGetPhysicalDeviceSurfaceFormats =
            unsafe { get_instance_proc_addr!(instance, "vkGetPhysicalDeviceSurfaceFormats") };
        let                                            fpGetPhysicalDeviceSurfacePresentModes:
                                 vk::khr::surface::PFN_vkGetPhysicalDeviceSurfacePresentModes =
            unsafe { get_instance_proc_addr!(instance, "vkGetPhysicalDeviceSurfacePresentModes") };
        let                                            fpCreateSwapchain:
                               vk::khr::swapchain::PFN_vkCreateSwapchain =
            unsafe { get_device_proc_addr!(device,     "vkCreateSwapchain") };
        let                                            fpDestroySwapchain:
                               vk::khr::swapchain::PFN_vkDestroySwapchain =
            unsafe { get_device_proc_addr!(device,     "vkDestroySwapchain") };
        let                                            fpGetSwapchainImages:
                               vk::khr::swapchain::PFN_vkGetSwapchainImages =
            unsafe { get_device_proc_addr!(device,     "vkGetSwapchainImages") };
        let                                            fpAcquireNextImage:
                               vk::khr::swapchain::PFN_vkAcquireNextImage =
            unsafe { get_device_proc_addr!(device,     "vkAcquireNextImage") };
        let                                            fpQueuePresent:
                               vk::khr::swapchain::PFN_vkQueuePresent =
            unsafe { get_device_proc_addr!(device,     "vkQueuePresent") };

        // ## initSurface

        let surface_create_info = vk::khr::xcb_surface::CreateInfo {
            sType: vk::StructureType::XCB_SURFACE_CREATE_INFO,
            connection: connection,
            window: window,
            pNext: null(),
            flags: Default::default(),
        };
        let surface = unsafe {
            let mut surface = mem::uninitialized();
            vksuccess(vk::khr::xcb_surface::vkCreateXcbSurfaceKHR(instance, &surface_create_info, null(), &mut surface));
            surface
        };

        let mut queue_count = 0u32;
        unsafe {
            vk::vkGetPhysicalDeviceQueueFamilyProperties(physical_device, &mut queue_count,
                                                         null_mut());
        }
        assert!(queue_count >= 1);

        let mut queue_props: Vec<vk::QueueFamilyProperties> = Vec::with_capacity(queue_count as usize);
        unsafe {
            vk::vkGetPhysicalDeviceQueueFamilyProperties(physical_device, &mut queue_count,
                                                         queue_props.as_mut_ptr());
            queue_props.set_len(queue_count as usize);
        }

        let mut supports_present: Vec<vk::Bool32> = vec![vk::FALSE; queue_count as usize];
        for i in 0..queue_count {
            unsafe {
                fpGetPhysicalDeviceSurfaceSupport.unwrap()(
                        physical_device, i, surface, &mut supports_present[i as usize]);
            }
        }

        let mut graphics_queue_node_index = std::u32::MAX;
        let mut present_queue_node_index = std::u32::MAX;
        for i in 0..queue_count {
            if (queue_props[i as usize].queueFlags & vk::QueueFlag::GRAPHICS).0 > 0 {
                if graphics_queue_node_index == std::u32::MAX {
                    graphics_queue_node_index = i;
                }
                if supports_present[i as usize] == vk::TRUE {
                    graphics_queue_node_index = i;
                    present_queue_node_index = i;
                    break;
                }
            }
        }
        if present_queue_node_index == std::u32::MAX {
            for i in 0..queue_count {
                if supports_present[i as usize] == vk::TRUE {
                    present_queue_node_index = i;
                    break;
                }
            }
        }

        assert!(graphics_queue_node_index != std::u32::MAX);
        assert!(present_queue_node_index != std::u32::MAX);
        assert!(graphics_queue_node_index == present_queue_node_index);

        let mut format_count = 0u32;
        unsafe {
            vksuccess(fpGetPhysicalDeviceSurfaceFormats.unwrap()(
                    physical_device, surface, &mut format_count, null_mut()));
        }
        assert!(format_count > 0);

        let mut surface_formats: Vec<vk::khr::surface::Format> = Vec::with_capacity(format_count as usize);
        unsafe {
            vksuccess(fpGetPhysicalDeviceSurfaceFormats.unwrap()(
                    physical_device, surface, &mut format_count, surface_formats.as_mut_ptr()));
            surface_formats.set_len(format_count as usize);
        }

        let color_format =
            if format_count == 1 && surface_formats[0].format == vk::Format::UNDEFINED {
                vk::Format::B8G8R8A8_UNORM
            } else {
                surface_formats[0].format
            };
        let color_space = surface_formats[0].colorSpace;

        Swapchain {
            instance: instance,
            device: device,
            physical_device: physical_device,
            surface: surface,
            fpGetPhysicalDeviceSurfaceSupport:      fpGetPhysicalDeviceSurfaceSupport,
            fpGetPhysicalDeviceSurfaceCapabilities: fpGetPhysicalDeviceSurfaceCapabilities,
            fpGetPhysicalDeviceSurfaceFormats:      fpGetPhysicalDeviceSurfaceFormats,
            fpGetPhysicalDeviceSurfacePresentModes: fpGetPhysicalDeviceSurfacePresentModes,
            fpCreateSwapchain:                      fpCreateSwapchain,
            fpDestroySwapchain:                     fpDestroySwapchain,
            fpGetSwapchainImages:                   fpGetSwapchainImages,
            fpAcquireNextImage:                     fpAcquireNextImage,
            fpQueuePresent:                         fpQueuePresent,
            color_format: color_format,
            color_space: color_space,
            swapchain: None,
            image_count: 0,
            images: vec![],
            buffers: vec![],
            queue_node_index: graphics_queue_node_index,
        }
    }

    pub fn create(&mut self, cmd_buffer: vk::CommandBuffer, width: &mut u32, height: &mut u32) {
        let old_swapchain = if let Some(s) = self.swapchain { s } else { null_mut() };
        let mut surf_caps = vk::khr::surface::Capabilities::default();
        unsafe {
            vksuccess(self.fpGetPhysicalDeviceSurfaceCapabilities.unwrap()(
                    self.physical_device, self.surface, &mut surf_caps));
        }

        let mut present_mode_count = 0u32;
        unsafe {
            vksuccess(self.fpGetPhysicalDeviceSurfacePresentModes.unwrap()(
                    self.physical_device, self.surface,
                    &mut present_mode_count, null_mut()));
        }
        assert!(present_mode_count > 0);

        let mut present_modes = Vec::with_capacity(present_mode_count as usize);
        unsafe {
            vksuccess(self.fpGetPhysicalDeviceSurfacePresentModes.unwrap()(
                    self.physical_device, self.surface,
                    &mut present_mode_count, present_modes.as_mut_ptr()));
            present_modes.set_len(present_mode_count as usize);
        }

        let swapchain_extent = if surf_caps.currentExtent.width == std::u32::MAX {
            vk::Extent2D {
                width: *width,
                height: *height,
            }
        } else {
            *width = surf_caps.currentExtent.width;
            *height = surf_caps.currentExtent.height;
            surf_caps.currentExtent
        };

        // Prefer mailbox mode if present, it's the lowest latency non-tearing present  mode
        let mut swapchain_present_mode = vk::khr::surface::PresentMode::FIFO;
        for p in present_modes {
            if p == vk::khr::surface::PresentMode::MAILBOX {
                swapchain_present_mode = p;
                break;
            }
            if swapchain_present_mode != vk::khr::surface::PresentMode::MAILBOX &&
                    p == vk::khr::surface::PresentMode::IMMEDIATE {
                swapchain_present_mode = p;
            }
        }

        // Determine the number of images
        let mut desired_number_of_swapchain_images = surf_caps.minImageCount + 1;
        if surf_caps.maxImageCount > 0 &&
                desired_number_of_swapchain_images > surf_caps.maxImageCount {
            desired_number_of_swapchain_images = surf_caps.maxImageCount;
        }

        let pre_transform =
            if (surf_caps.supportedTransforms & vk::khr::surface::TransformFlag::IDENTITY).0 > 0 {
                vk::khr::surface::TransformFlag::IDENTITY
            } else {
                surf_caps.currentTransform
            };

        let swapchain_ci = vk::khr::swapchain::CreateInfo {
            sType: vk::StructureType::SWAPCHAIN_CREATE_INFO,
            pNext: null(),
            surface: self.surface,
            minImageCount: desired_number_of_swapchain_images,
            imageFormat: self.color_format,
            imageColorSpace: self.color_space,
            imageExtent: swapchain_extent,
            imageUsage: vk::ImageUsageFlag::COLOR_ATTACHMENT.into(),
            preTransform: pre_transform,
            imageArrayLayers: 1,
            imageSharingMode: vk::SharingMode::EXCLUSIVE,
            queueFamilyIndexCount: 0,
            pQueueFamilyIndices: null(),
            presentMode: swapchain_present_mode,
            oldSwapchain: old_swapchain,
            clipped: vk::TRUE,
            compositeAlpha: vk::khr::surface::CompositeAlphaFlag::OPAQUE,
            flags: Default::default(),
        };

        // If an existing swapchain is re-created, destroy the old swapchain
        // This also cleans up all the presentable images
        unsafe {
            let mut swapchain = mem::uninitialized();
            vksuccess(self.fpCreateSwapchain.unwrap()(
                    self.device, &swapchain_ci, null(), &mut swapchain));
            self.swapchain = Some(swapchain);
        }

        if old_swapchain != null_mut() {
            unsafe {
                self.fpDestroySwapchain.unwrap()(self.device, old_swapchain, null());
            }
        }

        // Get the swapchain images
        let mut image_count = 0u32;
        unsafe {
            vksuccess(self.fpGetSwapchainImages.unwrap()(
                    self.device, self.swapchain.unwrap(),
                    &mut image_count, null_mut()));
        }

        self.images.reserve_exact(image_count as usize);
        unsafe {
            vksuccess(self.fpGetSwapchainImages.unwrap()(
                    self.device, self.swapchain.unwrap(),
                    &mut image_count, self.images.as_mut_ptr()));
            self.images.set_len(image_count as usize);
        }


        // Get the swap chain buffers containing the image and imageview
        self.buffers.reserve_exact(image_count as usize);
        for i in 0..image_count as usize {
            let color_attachment_view = vk::ImageViewCreateInfo {
                sType: vk::StructureType::IMAGE_VIEW_CREATE_INFO,
                pNext: null(),
                format: self.color_format,
                components: vk::ComponentMapping::default(),
                subresourceRange: vk::ImageSubresourceRange {
                    aspectMask: vk::ImageAspectFlag::COLOR.into(),
                    baseMipLevel: 0,
                    levelCount: 1,
                    baseArrayLayer: 0,
                    layerCount: 1,
                },
                viewType: vk::ImageViewType::E_2D,
                flags: Default::default(),
                image: self.images[i],
            };

            self.buffers[i].image = self.images[i];

            // Transform images from initial (undefined) to present layout
            ::tools::set_image_layout(cmd_buffer, self.buffers[i].image,
                                      vk::ImageAspectFlag::COLOR.into(),
                                      vk::ImageLayout::UNDEFINED,
                                      vk::ImageLayout::PRESENT_SRC);

            unsafe {
                vksuccess(vk::vkCreateImageView(
                        self.device, &color_attachment_view, null(), &mut self.buffers[i].view));
            }
        }
    }

    pub fn acquire_next_image(&self, present_complete_semaphore: vk::Semaphore)
                              -> Result<u32, vk::Result> {
        let mut current_buffer = 0u32;
        let res = unsafe {
            self.fpAcquireNextImage.unwrap()(self.device, self.swapchain.unwrap(),
                                             std::u64::MAX,
                                             present_complete_semaphore,
                                             null_mut(), &mut current_buffer)
        };
        if res == vk::Result::SUCCESS {
            Ok(current_buffer)
        } else {
            Err(res)
        }
    }

    pub fn queue_present(&self, queue: vk::Queue, current_buffer: u32,
                         wait_semaphore: vk::Semaphore) -> vk::Result {
        let present_info = vk::khr::swapchain::PresentInfo {
            sType: vk::StructureType::PRESENT_INFO,
            pNext: null(),
            swapchainCount: 1,
            pSwapchains: &self.swapchain.unwrap(),
            pImageIndices: &current_buffer,
            pWaitSemaphores: &wait_semaphore,
            waitSemaphoreCount: if wait_semaphore == null_mut() { 0 } else { 1 },
            pResults: null_mut(),
        };
        unsafe { self.fpQueuePresent.unwrap()(queue, &present_info) }
    }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        for i in 0..self.image_count as usize {
            unsafe {
                vk::vkDestroyImageView(self.device, self.buffers[i].view, null());
            }
        }
        unsafe {
            self.fpDestroySwapchain.unwrap()(self.device, self.swapchain.unwrap(), null());
            vk::khr::surface::vkDestroySurface(self.instance, self.surface, null());
        }
    }
}

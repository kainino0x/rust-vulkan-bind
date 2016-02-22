// Modeled after:
// https://github.com/SaschaWillems/Vulkan/blob/master/base/vulkantools.cpp
// Most comments are copied verbatim from vulkantools.cpp.

pub mod initializers;

use std;
use std::io::{self, Read};
use std::ptr::{null, null_mut};

use vulkan_bind::vk;

fn cstr(s: &[i8; 256]) -> &str {
    unsafe { std::str::from_utf8_unchecked(std::mem::transmute::<_, &[u8; 256]>(s)) }
}

fn u32_bytes(v: u32) -> [u8; 4] {
    unsafe { std::mem::transmute(v) }
}

pub fn vkassert(result: vk::Result) {
    assert_eq!(result, vk::Result::SUCCESS);
}

pub fn check_global_extension_present(extension_name: &str) -> bool {
    let mut extension_count = 0u32;
    unsafe {
        vk::vkEnumerateInstanceExtensionProperties(null(), &mut extension_count, null_mut());
    }

    let mut extensions: Vec<vk::ExtensionProperties> = Vec::with_capacity(extension_count as usize);
    unsafe {
        vk::vkEnumerateInstanceExtensionProperties(null(), &mut extension_count, extensions.as_mut_ptr());
        extensions.set_len(extension_count as usize);
    }

    extensions.into_iter().any(|ext| extension_name == cstr(&ext.extensionName))
}

pub fn check_device_extension_present(physical_device: vk::PhysicalDevice,
                                      extension_name: &str) -> bool {
    let mut extension_count = 0u32;
    unsafe {
        vk::vkEnumerateDeviceExtensionProperties(physical_device, null(),
                                                 &mut extension_count, null_mut());
    }

    let mut extensions = Vec::with_capacity(extension_count as usize);
    unsafe {
        vk::vkEnumerateDeviceExtensionProperties(physical_device, null(),
                                                 &mut extension_count, extensions.as_mut_ptr());
        extensions.set_len(extension_count as usize);
    }

    extensions.into_iter().any(|ext| extension_name == cstr(&ext.extensionName))
}

pub fn get_supported_depth_format(physical_device: vk::PhysicalDevice) -> Option<vk::Format> {
    // Since all depth formats may be optional, we need to find a suitable depth format to use
    // Start with the highest precision packed format
    let depth_formats = [
        vk::Format::D32_SFLOAT_S8_UINT,
        vk::Format::D32_SFLOAT,
        vk::Format::D24_UNORM_S8_UINT,
        vk::Format::D16_UNORM_S8_UINT,
        vk::Format::D16_UNORM,
    ];

    depth_formats.into_iter().find(|&&format| {
        let mut format_props = Default::default();
        unsafe {
            vk::vkGetPhysicalDeviceFormatProperties(physical_device, format, &mut format_props);
            // Format must support depth stencil attachment for optimal tiling
            (format_props.optimalTilingFeatures &
             vk::FormatFeatureFlag::DEPTH_STENCIL_ATTACHMENT).into()
        }
    }).cloned()
}

// Create an image memory barrier for changing the layout of
// an image and put it into an active command buffer
// See chapter 11.4 "Image Layout" for details
//todo : rename
pub fn set_image_layout(cmd_buffer: vk::CommandBuffer, image: vk::Image,
                        aspect_mask: vk::ImageAspectFlags,
                        old_layout: vk::ImageLayout, new_layout: vk::ImageLayout) {
    use vulkan_bind::vk::AccessFlag;

    // Create an image barrier object
    let mut image_mem_barrier = initializers::imageMemoryBarrier();
    image_mem_barrier.oldLayout = old_layout;
    image_mem_barrier.newLayout = new_layout;
    image_mem_barrier.image = image;
    image_mem_barrier.subresourceRange.aspectMask = aspect_mask;
    image_mem_barrier.subresourceRange.baseMipLevel = 0;
    image_mem_barrier.subresourceRange.levelCount = 1;
    image_mem_barrier.subresourceRange.layerCount = 1;

    // Source layouts (old)

    if old_layout == vk::ImageLayout::UNDEFINED {
        // Undefined layout
        // Only allowed as initial layout!
        // Make sure any writes to the image have been finished
        image_mem_barrier.srcAccessMask = AccessFlag::HOST_WRITE | AccessFlag::TRANSFER_WRITE;
    }

    if old_layout == vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL {
        // Old layout is color attachment
        // Make sure any writes to the color buffer have been finished
        image_mem_barrier.srcAccessMask = AccessFlag::COLOR_ATTACHMENT_WRITE.into();
    }

    if old_layout == vk::ImageLayout::TRANSFER_SRC_OPTIMAL {
        // Old layout is transfer source
        // Make sure any reads from the image have been finished
        image_mem_barrier.srcAccessMask = AccessFlag::TRANSFER_READ.into();
    }

    if old_layout == vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL {
        // Old layout is shader read (sampler, input attachment)
        // Make sure any shader reads from the image have been finished
        image_mem_barrier.srcAccessMask = AccessFlag::SHADER_READ.into();
    }

    // Target layouts (new)

    // New layout is transfer destination (copy, blit)
    // Make sure any copyies to the image have been finished
    if new_layout == vk::ImageLayout::TRANSFER_DST_OPTIMAL {
        image_mem_barrier.dstAccessMask = AccessFlag::TRANSFER_WRITE.into();
    }

    // New layout is transfer source (copy, blit)
    // Make sure any reads from and writes to the image have been finished
    if new_layout == vk::ImageLayout::TRANSFER_SRC_OPTIMAL {
        image_mem_barrier.srcAccessMask = image_mem_barrier.srcAccessMask |
            AccessFlag::TRANSFER_READ;
        image_mem_barrier.dstAccessMask = AccessFlag::TRANSFER_READ.into();
    }

    // New layout is color attachment
    // Make sure any writes to the color buffer hav been finished
    if new_layout == vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL {
        image_mem_barrier.dstAccessMask = AccessFlag::COLOR_ATTACHMENT_WRITE.into();
        image_mem_barrier.srcAccessMask = AccessFlag::TRANSFER_READ.into();
    }

    // New layout is depth attachment
    // Make sure any writes to depth/stencil buffer have been finished
    if new_layout == vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL {
        image_mem_barrier.dstAccessMask = image_mem_barrier.dstAccessMask |
            AccessFlag::DEPTH_STENCIL_ATTACHMENT_WRITE;
    }

    // New layout is shader read (sampler, input attachment)
    // Make sure any writes to the image have been finished
    if new_layout == vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL {
        image_mem_barrier.srcAccessMask = AccessFlag::HOST_WRITE | AccessFlag::TRANSFER_WRITE;
        image_mem_barrier.dstAccessMask = AccessFlag::SHADER_READ.into();
    }

    // Put barrier on top
    let src_stage_flags: vk::PipelineStageFlags = vk::PipelineStageFlag::TOP_OF_PIPE.into();
    let dst_stage_flags: vk::PipelineStageFlags = vk::PipelineStageFlag::TOP_OF_PIPE.into();

    // Put barrier inside setup command buffer
    unsafe {
        vk::vkCmdPipelineBarrier(cmd_buffer, src_stage_flags, dst_stage_flags,
                                 Default::default(), 0, null(), 0, null(),
                                 1, &image_mem_barrier);
    }
}

pub fn read_text_file(filename: &str) -> io::Result<String> {
    let mut f = try!(std::fs::File::open(filename));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s)
}

/// Load a binary file into a buffer (e.g. SPIR-V)
pub fn read_binary_file(filename: &str) -> io::Result<Vec<u8>> {
    let mut f = try!(std::fs::File::open(filename));
    let mut v = Vec::new();
    try!(f.read_to_end(&mut v));
    Ok(v)
}

pub fn load_shader(filename: &str, device: vk::Device,
                   /*stage: vk::ShaderStageFlag,*/) -> io::Result<vk::ShaderModule> {
    let shader_code = try!(read_binary_file(filename));

    let module_create_info = vk::ShaderModuleCreateInfo {
        sType: vk::StructureType::SHADER_MODULE_CREATE_INFO,
        pNext: null(),
        codeSize: shader_code.len(),
        pCode: unsafe { std::mem::transmute(shader_code.as_ptr()) },
        flags: Default::default(),
    };

    unsafe {
        let mut shader_module = std::mem::uninitialized();
        vkassert(vk::vkCreateShaderModule(device, &module_create_info, null(), &mut shader_module));
        Ok(shader_module)
    }
}

pub fn load_shader_glsl(filename: &str, device: vk::Device,
                        stage: vk::ShaderStageFlag) -> io::Result<vk::ShaderModule> {
    let mut shader_code = try!(read_binary_file(filename));

    let mut shader_data: Vec<u8> = Vec::with_capacity(12 + shader_code.len() + 1);
    shader_data.extend_from_slice(&u32_bytes(0x07230203u32));
    shader_data.extend_from_slice(&u32_bytes(0u32));
    shader_data.extend_from_slice(&u32_bytes(stage.0));
    shader_data.append(&mut shader_code);
    shader_data.push(0u8);
    assert_eq!(shader_data.len(), 12 + shader_code.len() + 1);

    let module_create_info = vk::ShaderModuleCreateInfo {
        sType: vk::StructureType::SHADER_MODULE_CREATE_INFO,
        pNext: null(),
        codeSize: 3 * std::mem::size_of::<u32>() + shader_code.len() + 1,
        pCode: unsafe { std::mem::transmute(shader_code.as_ptr()) },
        flags: Default::default(),
    };

    unsafe {
        let mut shader_module = std::mem::uninitialized();
        vkassert(vk::vkCreateShaderModule(device, &module_create_info, null(), &mut shader_module));
        Ok(shader_module)
    }
}

pub fn pre_present_barrier(present_image: vk::Image) -> vk::ImageMemoryBarrier {
    let mut image_mem_barrier = initializers::imageMemoryBarrier();
    image_mem_barrier.srcAccessMask = vk::AccessFlag::COLOR_ATTACHMENT_WRITE.into();
    image_mem_barrier.dstAccessMask = Default::default();
    image_mem_barrier.oldLayout = vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL;
    image_mem_barrier.newLayout = vk::ImageLayout::PRESENT_SRC;
    image_mem_barrier.srcQueueFamilyIndex = vk::QUEUE_FAMILY_IGNORED;
    image_mem_barrier.dstQueueFamilyIndex = vk::QUEUE_FAMILY_IGNORED;
    image_mem_barrier.subresourceRange = vk::ImageSubresourceRange {
        aspectMask: vk::ImageAspectFlag::COLOR.into(),
        baseMipLevel: 0, levelCount: 1,
        baseArrayLayer: 0, layerCount: 1,
    };
    image_mem_barrier.image = present_image;
    image_mem_barrier
}

pub fn post_present_barrier(present_image: vk::Image) -> vk::ImageMemoryBarrier{
    let mut image_mem_barrier = initializers::imageMemoryBarrier();
    image_mem_barrier.srcAccessMask = Default::default();
    image_mem_barrier.dstAccessMask = vk::AccessFlag::COLOR_ATTACHMENT_WRITE.into();
    image_mem_barrier.oldLayout = vk::ImageLayout::PRESENT_SRC;
    image_mem_barrier.newLayout = vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL;
    image_mem_barrier.srcQueueFamilyIndex = vk::QUEUE_FAMILY_IGNORED;
    image_mem_barrier.dstQueueFamilyIndex = vk::QUEUE_FAMILY_IGNORED;
    image_mem_barrier.subresourceRange = vk::ImageSubresourceRange {
        aspectMask: vk::ImageAspectFlag::COLOR.into(),
        baseMipLevel: 0, levelCount: 1,
        baseArrayLayer: 0, layerCount: 1,
    };
    image_mem_barrier.image = present_image;
    image_mem_barrier
}

/// Contains all vulkan objects required for a uniform data object
pub struct UniformData {
    pub buffer: vk::Buffer,
    pub memory: vk::DeviceMemory,
    pub descriptor: vk::DescriptorBufferInfo,
    pub alloc_size: u32,
}

pub fn destroy_uniform_data(device: vk::Device, uniform_data: UniformData) {
    unsafe {
        vk::vkDestroyBuffer(device, uniform_data.buffer, null());
        vk::vkFreeMemory(device, uniform_data.memory, null());
    }
}

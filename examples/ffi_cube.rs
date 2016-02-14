#![allow(unused_imports, dead_code, non_snake_case)]

extern crate cgmath;
extern crate vulkan;
extern crate xcb;

use vulkan::ffi::vulkan::*;
use vulkan::ffi::vk_ext_debug_report::*;
use vulkan::ffi::vk_sdk_platform::*;
use cgmath::*;

const DEMO_TEXTURE_COUNT: usize = 1;
const APP_SHORT_NAME: &'static str = "cube";
const APP_LONG_NAME: &'static str = "The Vulkan Cube Demo Program";

fn main() {
    let mut demo = Demo::init(std::env::args());
    demo.create_window();
    demo.init_vk_swapchain();

    demo.prepare();
    demo.run();

    demo.cleanup();
}

struct Demo<'a> {
    connection: Box<xcb::base::Connection>,
    screen: Box<xcb::xproto::Screen<'a>>,
    window: xcb::xproto::Window,
    atom_wm_delete_window: Box<xcb::xproto::InternAtomReply>,
    surface: VkSurfaceKHR,
    prepared: bool,
    use_staging_buffer: bool,

    inst: VkInstance,
    gpu: VkPhysicalDevice,
    device: VkDevice,
    queue: VkQueue,
    graphics_queue_node_index: u32,
    gpu_props: VkPhysicalDeviceProperties,
    queue_props: Box<VkQueueFamilyProperties>,
    memory_properties: VkPhysicalDeviceMemoryProperties,

    enabled_extension_count: u32,
    enabled_layer_count: u32,
    extension_names: Vec<&'static str>,
    device_validation_layers: Vec<&'static str>,

    width: i32,
    height: i32,
    format: VkFormat,
    color_space: VkColorSpaceKHR,

    fpGetPhysicalDeviceSurfaceSupportKHR:      PFN_vkGetPhysicalDeviceSurfaceSupportKHR,
    fpGetPhysicalDeviceSurfaceCapabilitiesKHR: PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR,
    fpGetPhysicalDeviceSurfaceFormatsKHR:      PFN_vkGetPhysicalDeviceSurfaceFormatsKHR,
    fpGetPhysicalDeviceSurfacePresentModesKHR: PFN_vkGetPhysicalDeviceSurfacePresentModesKHR,
    fpCreateSwapchainKHR:                      PFN_vkCreateSwapchainKHR,
    fpDestroySwapchainKHR:                     PFN_vkDestroySwapchainKHR,
    fpGetSwapchainImagesKHR:                   PFN_vkGetSwapchainImagesKHR,
    fpAcquireNextImageKHR:                     PFN_vkAcquireNextImageKHR,
    fpQueuePresentKHR:                         PFN_vkQueuePresentKHR,
    swapchainImageCount: u32,
    swapchain: VkSwapchainKHR,
    buffers: Box<SwapchainBuffers>,

    cmd_pool: VkCommandPool,

    depth: Depth,

    textures: [TextureObject; DEMO_TEXTURE_COUNT],

    uniform_data: UniformData,

    cmd: VkCommandBuffer,
    pipeline_layout: VkPipelineLayout,
    desc_layout: VkDescriptorSetLayout,
    pipelineCache: VkPipelineCache,
    render_pass: VkRenderPass,
    pipeline: VkPipeline,

    projection_matrix: Matrix4<f32>,
    view_matrix: Matrix4<f32>,
    model_matrix: Matrix4<f32>,

    spin_angle: f32,
    spin_increment: f32,
    pause: bool,

    vert_shader_module: VkShaderModule,
    frag_shader_module: VkShaderModule,

    desc_pool: VkDescriptorPool,
    desc_set: VkDescriptorSet,

    framebuffers: Box<VkFramebuffer>,

    quit: bool,
    curFrame: i32,
    frameCount: i32,
    validate: bool,
    use_break: bool,
    CreateDebugReportCallback: PFN_vkCreateDebugReportCallbackEXT,
    DestroyDebugReportCallback: PFN_vkDestroyDebugReportCallbackEXT,
    msg_callback: VkDebugReportCallbackEXT,
    DebugReportMessage: PFN_vkDebugReportMessageEXT,

    current_buffer: u32,
    queue_count: u32,
}

struct Depth {
    format: VkFormat,
    image: VkImage,
    mem_alloc: VkMemoryAllocateInfo,
    mem: VkDeviceMemory,
    view: VkImageView,
}

struct UniformData {
    buf: VkBuffer,
    mem_alloc: VkMemoryAllocateInfo,
    mem: VkDeviceMemory,
    buffer_info: VkDescriptorBufferInfo,
}

struct SwapchainBuffers {
    image: VkImage,
    cmd: VkCommandBuffer,
    view: VkImageView,
}

struct TextureObject {
    sampler: VkSampler,
    image: VkImage,
    imageLayout: VkImageLayout,
    mem_alloc: VkMemoryAllocateInfo,
    mem: VkDeviceMemory,
    view: VkImageView,
    tex_width: i32,
    tex_height: i32,
}

impl<'a> Demo<'a> {
    fn init(args: std::env::Args) -> Demo<'a> {
        let eye: Point3<f32> = Point3::new(0.0, 3.0, 5.0);
        let origin: Point3<f32> = Point3::origin();
        let up: Vector3<f32> = Vector3::unit_y();

        let mut demo: Demo<'a> = unsafe { std::mem::zeroed() };
        demo.frameCount = std::i32::MAX;

        for arg in args.skip(1) {
            if arg == "--use_staging" {
                demo.use_staging_buffer = true;
            } else if arg == "--break" {
                demo.use_break = true;
            } else if arg == "--validate" {
                demo.validate = true;
            } else if arg == "--c" {
                if demo.frameCount == std::i32::MAX {
                }
            } else {
                println!(
                    "Usage:\n  {} [--use_staging] \
                    [--validate] [--break] [--c <framecount>]",
                    APP_SHORT_NAME
                    );
                std::process::exit(1);
            }
        }

        demo.init_connection();
        demo.init_vk();

        demo.width = 500;
        demo.height = 500;

        demo.spin_angle = 0.01;
        demo.spin_increment = 0.01;
        demo.pause = false;

        demo.projection_matrix = perspective(Deg::new(45f32), 1.0, 0.1, 100.0);
        demo.view_matrix = Matrix4::look_at(eye, origin, up);
        demo.model_matrix = Matrix4::identity();

        demo
    }

    fn init_connection(&mut self) {
    }

    fn init_vk(&mut self) {
    }

    fn create_window(&mut self) {
    }

    fn init_vk_swapchain(&mut self) {
    }

    fn prepare(&mut self) {
    }

    fn run(&mut self) {
    }

    fn cleanup(&mut self) {
    }
}

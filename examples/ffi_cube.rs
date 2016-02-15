#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables)]
#![feature(box_syntax, raw)]

extern crate cgmath;
extern crate vulkan;
extern crate xcb;

use std::ffi;
use std::raw::Repr;
use cgmath::*;
use xcb::base as xcbb;
use xcb::xproto as xcbx;

use vulkan::ffi::vulkan::*;
use vulkan::ffi::vk_ext_debug_report::*;

const DEMO_TEXTURE_COUNT: usize = 1;
const APP_SHORT_NAME: &'static str = "cube";
const APP_LONG_NAME: &'static str = "The Vulkan Cube Demo Program";

fn vkassert(result: VkResult) {
    assert_eq!(result, Enum_VkResult::VK_SUCCESS);
}

fn main() {
    let mut demo = Demo::init(std::env::args());
    demo.create_window();
    demo.init_vk_swapchain();

    demo.prepare();
    demo.run();

    demo.cleanup();
}

struct Demo<'a> {
    cfg: DemoConfig,
    prepared: bool,
    width: i32,
    height: i32,
    projection_matrix: Matrix4<f32>,
    view_matrix: Matrix4<f32>,
    model_matrix: Matrix4<f32>,
    spin_angle: f32,
    pause: bool,
    quit: bool,
    curFrame: i32,
    validate: bool,

    current_buffer: u32,

    windowing: DemoWindowing<'a>,
    vk: DemoVk,
}

struct DemoVk {
    surface:                                   VkSurfaceKHR,
    inst:                                      VkInstance,
    gpu:                                       VkPhysicalDevice,
    device:                                    VkDevice,
    queue:                                     VkQueue,
    graphics_queue_node_index:                 u32,
    gpu_props:                                 VkPhysicalDeviceProperties,
    queue_props:                               Vec<VkQueueFamilyProperties>, // queue_count
    memory_properties:                         VkPhysicalDeviceMemoryProperties,

    extension_names:                           Vec<&'static str>,
    device_validation_layers:                  Vec<&'static str>,

    format:                                    VkFormat,
    color_space:                               VkColorSpaceKHR,

    swapchain:                                 VkSwapchainKHR,
    buffers:                                   Vec<SwapchainBuffers>, // swapchainImageCount

    cmd_pool:                                  VkCommandPool,

    depth:                                     Depth,

    textures:                                  [TextureObject; DEMO_TEXTURE_COUNT],

    uniform_data:                              UniformData,

    cmd:                                       VkCommandBuffer,
    pipeline_layout:                           VkPipelineLayout,
    desc_layout:                               VkDescriptorSetLayout,
    pipelineCache:                             VkPipelineCache,
    render_pass:                               VkRenderPass,
    pipeline:                                  VkPipeline,

    vert_shader_module:                        VkShaderModule,
    frag_shader_module:                        VkShaderModule,

    desc_pool:                                 VkDescriptorPool,
    desc_set:                                  VkDescriptorSet,

    framebuffers:                              Vec<VkFramebuffer>,

    fpGetPhysicalDeviceSurfaceSupportKHR:      PFN_vkGetPhysicalDeviceSurfaceSupportKHR,
    fpGetPhysicalDeviceSurfaceCapabilitiesKHR: PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR,
    fpGetPhysicalDeviceSurfaceFormatsKHR:      PFN_vkGetPhysicalDeviceSurfaceFormatsKHR,
    fpGetPhysicalDeviceSurfacePresentModesKHR: PFN_vkGetPhysicalDeviceSurfacePresentModesKHR,
    fpCreateSwapchainKHR:                      PFN_vkCreateSwapchainKHR,
    fpDestroySwapchainKHR:                     PFN_vkDestroySwapchainKHR,
    fpGetSwapchainImagesKHR:                   PFN_vkGetSwapchainImagesKHR,
    fpAcquireNextImageKHR:                     PFN_vkAcquireNextImageKHR,
    fpQueuePresentKHR:                         PFN_vkQueuePresentKHR,
    CreateDebugReportCallback:                 PFN_vkCreateDebugReportCallbackEXT,
    DestroyDebugReportCallback:                PFN_vkDestroyDebugReportCallbackEXT,
    msg_callback:                              VkDebugReportCallbackEXT,
    DebugReportMessage:                        PFN_vkDebugReportMessageEXT,
}

impl DemoVk {
    fn new(cfg: &DemoConfig) -> Self {
        let instance_validation_layers = vec![
            "VK_LAYER_LUNARG_threading",
            "VK_LAYER_LUNARG_mem_tracker",
            "VK_LAYER_LUNARG_object_tracker",
            "VK_LAYER_LUNARG_draw_state",
            "VK_LAYER_LUNARG_param_checker",
            "VK_LAYER_LUNARG_swapchain",
            "VK_LAYER_LUNARG_device_limits",
            "VK_LAYER_LUNARG_image",
            "VK_LAYER_GOOGLE_unique_objects",
        ];

        let device_validation_layers = vec![
            "VK_LAYER_LUNARG_threading",
            "VK_LAYER_LUNARG_mem_tracker",
            "VK_LAYER_LUNARG_object_tracker",
            "VK_LAYER_LUNARG_draw_state",
            "VK_LAYER_LUNARG_param_checker",
            "VK_LAYER_LUNARG_swapchain",
            "VK_LAYER_LUNARG_device_limits",
            "VK_LAYER_LUNARG_image",
            "VK_LAYER_GOOGLE_unique_objects",
        ];

        // Look for validation layers
        let mut instance_layer_count = 0u32;
        vkassert(unsafe {
            vkEnumerateInstanceLayerProperties(
                &mut instance_layer_count, std::ptr::null_mut())
        });

        let mut enabled_layer_count = 0;
        if instance_layer_count > 0 {
            let mut instance_layers: Vec<VkLayerProperties> =
                Vec::with_capacity(instance_layer_count as usize);
            unsafe {
                vkassert(vkEnumerateInstanceLayerProperties(
                    &mut instance_layer_count, instance_layers.as_mut_ptr()));
                instance_layers.set_len(instance_layer_count as usize);
            };

            if cfg.validate {
                assert!(Self::check_layers(
                    &device_validation_layers, &instance_layers));
                enabled_layer_count = device_validation_layers.len();
            }
        }

        // Look for instance extensions
        let mut instance_extension_count = 0u32;
        let mut surfaceExtFound = false;
        let mut platformSurfaceExtFound = false;
        let mut extension_names: Vec<&'static str> = Vec::new();
        unsafe {
            vkassert(vkEnumerateInstanceExtensionProperties(std::ptr::null(),
                &mut instance_extension_count, std::ptr::null_mut()));
        }
        if instance_extension_count > 0 {
            let mut instance_extensions: Vec<VkExtensionProperties> =
                Vec::with_capacity(instance_extension_count as usize);
            unsafe {
                vkassert(vkEnumerateInstanceExtensionProperties(std::ptr::null(),
                    &mut instance_extension_count, instance_extensions.as_mut_ptr()));
                instance_extensions.set_len(instance_extension_count as usize);
            }
            for ext in &instance_extensions {
                if carr_eq_str(&ext.extensionName, VK_KHR_SURFACE_EXTENSION_NAME) {
                    surfaceExtFound = true;
                    extension_names.push(VK_KHR_SURFACE_EXTENSION_NAME);
                }
                if carr_eq_str(&ext.extensionName, VK_KHR_XCB_SURFACE_EXTENSION_NAME) {
                    platformSurfaceExtFound = true;
                    extension_names.push(VK_KHR_XCB_SURFACE_EXTENSION_NAME);
                }
                if carr_eq_str(&ext.extensionName, VK_EXT_DEBUG_REPORT_EXTENSION_NAME) {
                    if cfg.validate {
                        extension_names.push(VK_EXT_DEBUG_REPORT_EXTENSION_NAME);
                    }
                }
            }
        }
        assert!(surfaceExtFound);
        assert!(platformSurfaceExtFound);

        let app = VkApplicationInfo {
            sType: Enum_VkStructureType::VK_STRUCTURE_TYPE_APPLICATION_INFO,
            pNext: std::ptr::null(),
            pApplicationName: APP_SHORT_NAME.repr().data as *const i8,
            applicationVersion: 0,
            pEngineName: APP_SHORT_NAME.repr().data as *const i8,
            engineVersion: 0,
            apiVersion: VK_API_VERSION,
        };

        let instance_validation_layers = vec_str_to_vec_ptr(&instance_validation_layers);
        let extension_names = vec_str_to_vec_ptr(&extension_names);
        let inst_info = VkInstanceCreateInfo {
            sType: Enum_VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            pNext: std::ptr::null(),
            pApplicationInfo: &app,
            enabledLayerCount: instance_validation_layers.len() as u32,
            ppEnabledLayerNames: if cfg.validate {
                instance_validation_layers.as_ptr()
            } else {
                std::ptr::null()
            },
            enabledExtensionCount: extension_names.len() as u32,
            ppEnabledExtensionNames: extension_names.as_ptr(),
            flags: 0,
        };

        let inst = unsafe {
            let mut inst: VkInstance = std::mem::uninitialized();
            vkassert(vkCreateInstance(&inst_info, std::ptr::null(), &mut inst));
            inst
        };

        let mut gpu_count = 0u32;
        // Make initial call to query gpu_count, then second call for gpu info
        unsafe {
            vkassert(vkEnumeratePhysicalDevices(inst, &mut gpu_count, std::ptr::null_mut()));
        }
        assert!(gpu_count > 0);

        let gpu = {
            let mut physical_devices: Vec<VkPhysicalDevice> = Vec::with_capacity(gpu_count as usize);
            unsafe {
                vkassert(vkEnumeratePhysicalDevices(inst, &mut gpu_count, physical_devices.as_mut_ptr()));
                physical_devices.set_len(gpu_count as usize);
            }
            // For cube demo we just grab the first physical device
            physical_devices[0]
        };

        // Look for validation layers
        enabled_layer_count = 0;
        let mut device_layer_count = 0u32;
        unsafe {
            vkassert(vkEnumerateDeviceLayerProperties(gpu, &mut device_layer_count, std::ptr::null_mut()));
        }

        if device_layer_count > 0 {
            let mut device_layers: Vec<VkLayerProperties> = Vec::with_capacity(device_layer_count as usize);
            unsafe {
                vkassert(vkEnumerateDeviceLayerProperties(gpu, &mut device_layer_count, device_layers.as_mut_ptr()));
                device_layers.set_len(device_layer_count as usize);
            }

            if cfg.validate {
                assert!(Self::check_layers(&device_validation_layers, &device_layers));
                enabled_layer_count = device_validation_layers.len();
            }
        }

        unimplemented!(); // TODO: keep working
    }

    /// Return true if all layer names specified in check_names
    /// can be found in given layer properties.
    fn check_layers(check_names: &Vec<&'static str>,
                    layers: &Vec<VkLayerProperties>) -> bool {
        for name in check_names {
            if !layers.iter().any(|p| { carr_eq_str(&p.layerName, *name) }) {
                println!("Cannot find layer: {}", name);
                return false;
            }
        }
        true
    }
}

fn cstr_eq_str(s: *const i8, t: &str) -> bool {
    unsafe { ffi::CStr::from_ptr(s) }.to_str().unwrap() == t
}

fn carr_eq_str(s: &[i8; 256], t: &str) -> bool {
    cstr_eq_str(&*s as *const i8, t)
}

fn vec_str_to_vec_ptr(v: &Vec<&'static str>) -> Vec<*const i8> {
    v.iter().map(|s| s.repr().data as *const i8).collect()
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

struct DemoConfig {
    use_staging_buffer: bool,
    use_break: bool,
    validate: bool,
    frameCount: i32,
    spin_increment: f32,
}

struct DemoWindowing<'a> {
    connection:            Box<xcbb::Connection>,
    screen:                xcbx::Screen<'a>,
    window:                Option<xcbx::Window>,
    atom_wm_delete_window: Option<Box<xcbx::InternAtomReply>>,
}

impl<'a> DemoWindowing<'a> {
    fn new() -> Self {
        let (connection, screen_num) = xcbb::Connection::connect();
        let screen = {
            let setup = connection.get_setup();
            let screen = setup.roots().nth(screen_num as usize).unwrap();
            unsafe {
                // Ignore the lifetimes on screen. I don't know why I have to
                // do this, precisely, but I'm having trouble with the lifetime
                // bounds in rust-xcb.
                std::mem::transmute::<xcbx::Screen, xcbx::Screen>(screen)
            }
        };

        DemoWindowing {
            connection: box connection,
            screen: screen,
            window: None,
            atom_wm_delete_window: None,
        }
    }
}

impl DemoConfig {
    fn new() -> Self { DemoConfig {
        use_staging_buffer: false,
        use_break: false,
        validate: false,
        frameCount: std::i32::MAX,
        spin_increment: 0.01,
    } }
}

impl<'a> Demo<'a> {
    fn init(args: std::env::Args) -> Self {
        let eye: Point3<f32> = Point3::new(0.0, 3.0, 5.0);
        let origin: Point3<f32> = Point3::origin();
        let up: Vector3<f32> = Vector3::unit_y();

        let mut cfg = DemoConfig::new();

        cfg.frameCount = std::i32::MAX;

        for arg in args.skip(1) {
            if arg == "--use_staging" {
                cfg.use_staging_buffer = true;
            } else if arg == "--break" {
                cfg.use_break = true;
            } else if arg == "--validate" {
                cfg.validate = true;
            } else if arg == "--c" {
                // TODO: this is supposed to take an argument. Use getopts.
            } else {
                println!(
                    "Usage:\n  {} [--use_staging] \
                    [--validate] [--break] [--c <framecount>]",
                    APP_SHORT_NAME
                    );
                std::process::exit(1);
            }
        }

        let windowing = DemoWindowing::new();

        //cfg.width = 500;
        //cfg.height = 500;

        //cfg.spin_angle = 0.01;
        //cfg.spin_increment = 0.01;
        //cfg.pause = false;

        //cfg.projection_matrix = perspective(Deg::new(45f32), 1.0, 0.1, 100.0);
        //cfg.view_matrix = Matrix4::look_at(eye, origin, up);
        //cfg.model_matrix = Matrix4::identity();

        //{ // init_connection
        //    // TODO: init_connection
        //}
        //{ // init_vk
        //    // TODO: init_vk
        //}

        //Demo {
        //    cfg: cfg,
        //    windowing: windowing,
        //    surface: surface,
        //    prepared: true,

        //    inst: inst,
        //    gpu: gpu,
        //    device: device,

        //}

        unimplemented!();
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

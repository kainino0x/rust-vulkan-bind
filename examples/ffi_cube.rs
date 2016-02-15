#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables)]
#![feature(box_syntax, raw)]

extern crate cgmath;
extern crate vulkan;
extern crate xcb;

use std::ffi;
use std::raw::Repr;
use std::ptr::{null, null_mut};
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
    demo_swapchain:                            DemoVkSwapchain,
    inst:                                      VkInstance,
    gpu:                                       VkPhysicalDevice,
    device:                                    VkDevice,
    graphics_queue_node_index:                 u32,
    gpu_props:                                 VkPhysicalDeviceProperties,
    queue_props:                               Vec<VkQueueFamilyProperties>, // queue_count

    extension_names:                           Vec<&'static str>,
    device_validation_layers:                  Vec<&'static str>,

    fpGetPhysicalDeviceSurfaceSupportKHR:      PFN_vkGetPhysicalDeviceSurfaceSupportKHR,
    fpGetPhysicalDeviceSurfaceCapabilitiesKHR: PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR,
    fpGetPhysicalDeviceSurfaceFormatsKHR:      PFN_vkGetPhysicalDeviceSurfaceFormatsKHR,
    fpGetPhysicalDeviceSurfacePresentModesKHR: PFN_vkGetPhysicalDeviceSurfacePresentModesKHR,
    fpGetSwapchainImagesKHR:                   PFN_vkGetSwapchainImagesKHR,
}

struct DemoVkSwapchain {
    surface:                                   VkSurfaceKHR,
    queue:                                     VkQueue,
    memory_properties:                         VkPhysicalDeviceMemoryProperties,
    color_space:                               VkColorSpaceKHR,

    format:                                    VkFormat,
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

    fpCreateSwapchainKHR:                      PFN_vkCreateSwapchainKHR,
    fpDestroySwapchainKHR:                     PFN_vkDestroySwapchainKHR,
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
                &mut instance_layer_count, null_mut())
        });

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
            }
        }

        // Look for instance extensions
        let mut instance_extension_count = 0u32;
        let mut surfaceExtFound = false;
        let mut platformSurfaceExtFound = false;
        let mut extension_names: Vec<&'static str> = Vec::new();
        unsafe {
            vkassert(vkEnumerateInstanceExtensionProperties(null(),
                &mut instance_extension_count, null_mut()));
        }
        if instance_extension_count > 0 {
            let mut instance_extensions: Vec<VkExtensionProperties> =
                Vec::with_capacity(instance_extension_count as usize);
            unsafe {
                vkassert(vkEnumerateInstanceExtensionProperties(null(),
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
            pNext: null(),
            pApplicationName: str_to_ptr(APP_SHORT_NAME),
            applicationVersion: 0,
            pEngineName: str_to_ptr(APP_SHORT_NAME),
            engineVersion: 0,
            apiVersion: VK_API_VERSION,
        };

        let instance_validation_layers = vec_str_to_vec_ptr(&instance_validation_layers);
        let extension_names = vec_str_to_vec_ptr(&extension_names);
        let inst_info = VkInstanceCreateInfo {
            sType: Enum_VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            pNext: null(),
            pApplicationInfo: &app,
            enabledLayerCount:
                if cfg.validate {
                    instance_validation_layers.len() as u32
                } else {
                    0
                },
            ppEnabledLayerNames:
                if cfg.validate {
                    instance_validation_layers.as_ptr()
                } else {
                    null() // FIXME: apparently this causes a crash.
                },
            enabledExtensionCount: extension_names.len() as u32,
            ppEnabledExtensionNames: extension_names.as_ptr(),
            flags: 0,
        };

        let inst = unsafe {
            let mut inst: VkInstance = std::mem::uninitialized();
            vkassert(vkCreateInstance(&inst_info, null(), &mut inst));
            inst
        };

        let mut gpu_count = 0u32;
        // Make initial call to query gpu_count, then second call for gpu info
        unsafe {
            vkassert(vkEnumeratePhysicalDevices(inst, &mut gpu_count, null_mut()));
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
        let mut device_layer_count = 0u32;
        unsafe {
            vkassert(vkEnumerateDeviceLayerProperties(gpu, &mut device_layer_count, null_mut()));
        }

        if device_layer_count > 0 {
            let mut device_layers: Vec<VkLayerProperties> = Vec::with_capacity(device_layer_count as usize);
            unsafe {
                vkassert(vkEnumerateDeviceLayerProperties(gpu, &mut device_layer_count, device_layers.as_mut_ptr()));
                device_layers.set_len(device_layer_count as usize);
            }

            if cfg.validate {
                assert!(Self::check_layers(&device_validation_layers, &device_layers));
            }
        }

        // Look for device extensions
        let mut device_extension_count = 0u32;
        let mut swapchainExtFound = false;
        let mut extension_names: Vec<&'static str> = Vec::new();
        unsafe {
            vkassert(vkEnumerateDeviceExtensionProperties(
                    gpu, null(), &mut device_extension_count, null_mut()));
        }

        if device_extension_count > 0 {
            let mut device_extensions =
                Vec::with_capacity(device_extension_count as usize);
            unsafe {
                vkassert(vkEnumerateDeviceExtensionProperties(
                        gpu, null(), &mut device_extension_count,
                        device_extensions.as_mut_ptr()));
                device_extensions.set_len(device_extension_count as usize);
            }

            for ext in device_extensions {
                if carr_eq_str(&ext.extensionName, VK_KHR_SWAPCHAIN_EXTENSION_NAME) {
                    swapchainExtFound = true;
                    extension_names.push(VK_KHR_SWAPCHAIN_EXTENSION_NAME);
                }
            }
        }
        assert!(swapchainExtFound);

        if cfg.validate {
            // TODO
        }
        let mut gpu_props: VkPhysicalDeviceProperties =
            unsafe { std::mem::uninitialized() };
        unsafe {
            vkGetPhysicalDeviceProperties(gpu, &mut gpu_props);
        }

        // Call with null data to get count
        let mut queue_count = 0u32;
        unsafe {
            vkGetPhysicalDeviceQueueFamilyProperties(
                gpu, &mut queue_count, null_mut());
        }
        assert!(queue_count >= 1);
        let mut queue_props = Vec::with_capacity(queue_count as usize);
        unsafe {
            vkGetPhysicalDeviceQueueFamilyProperties(
                gpu, &mut queue_count, queue_props.as_mut_ptr());
            queue_props.set_len(queue_count as usize);
        }
        // Find a queue that supports gfx
        let mut gfx_queue_found = false;
        for prop in &queue_props {
            if 0 != (prop.queueFlags & (Enum_VkQueueFlagBits::VK_QUEUE_GRAPHICS_BIT as u32)) {
                gfx_queue_found = true;
                break;
            }
        }
        assert!(gfx_queue_found);
        // Query fine-grained feature support for this device.
        //   If app has specific feature requirements then it should check
        //   supported features based on this query.
        let mut physDevFeatures: VkPhysicalDeviceFeatures = unsafe { std::mem::uninitialized() };
        unsafe {
            vkGetPhysicalDeviceFeatures(gpu, &mut physDevFeatures);
        }

        let fpGetPhysicalDeviceSurfaceSupportKHR      = unsafe { std::mem::transmute::<_, PFN_vkGetPhysicalDeviceSurfaceSupportKHR     >(vkGetInstanceProcAddr(inst, str_to_ptr("fpGetPhysicalDeviceSurfaceSupportKHR"))      ) };
        let fpGetPhysicalDeviceSurfaceCapabilitiesKHR = unsafe { std::mem::transmute::<_, PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR>(vkGetInstanceProcAddr(inst, str_to_ptr("fpGetPhysicalDeviceSurfaceCapabilities)KHR"))) };
        let fpGetPhysicalDeviceSurfaceFormatsKHR      = unsafe { std::mem::transmute::<_, PFN_vkGetPhysicalDeviceSurfaceFormatsKHR     >(vkGetInstanceProcAddr(inst, str_to_ptr("fpGetPhysicalDeviceSurfaceFormatsKHR"))      ) };
        let fpGetPhysicalDeviceSurfacePresentModesKHR = unsafe { std::mem::transmute::<_, PFN_vkGetPhysicalDeviceSurfacePresentModesKHR>(vkGetInstanceProcAddr(inst, str_to_ptr("fpGetPhysicalDeviceSurfacePresentModes)KHR"))) };
        let fpGetSwapchainImagesKHR                   = unsafe { std::mem::transmute::<_, PFN_vkGetSwapchainImagesKHR                  >(vkGetInstanceProcAddr(inst, str_to_ptr("fpGetSwapchainImagesKHR"))                   ) };

        DemoVk {
            demo_swapchain: unsafe { std::mem::uninitialized() },
            inst: inst,
            gpu: gpu,
            device: unsafe { std::mem::uninitialized() },
            graphics_queue_node_index: std::u32::MAX,
            gpu_props: gpu_props,
            queue_props: queue_props,
            extension_names: extension_names,
            device_validation_layers: device_validation_layers,
            fpGetPhysicalDeviceSurfaceSupportKHR:      fpGetPhysicalDeviceSurfaceSupportKHR,
            fpGetPhysicalDeviceSurfaceCapabilitiesKHR: fpGetPhysicalDeviceSurfaceCapabilitiesKHR,
            fpGetPhysicalDeviceSurfaceFormatsKHR:      fpGetPhysicalDeviceSurfaceFormatsKHR,
            fpGetPhysicalDeviceSurfacePresentModesKHR: fpGetPhysicalDeviceSurfacePresentModesKHR,
            fpGetSwapchainImagesKHR:                   fpGetSwapchainImagesKHR,
        }
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

fn str_to_ptr(s: &'static str) -> *const i8 {
    s.repr().data as *const i8
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

        let demo_vk = DemoVk::new(&cfg);

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

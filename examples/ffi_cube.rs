#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables)]
#![feature(box_syntax, raw)]

extern crate cgmath;
extern crate vulkan;
extern crate xcb;

use std::ffi;
use std::mem;
use std::ptr::{null, null_mut};
use std::raw::Repr;
use cgmath::*;

use vulkan::ffi::vulkan::*;
use vulkan::ffi::vk_ext_debug_report::*;

const DEMO_TEXTURE_COUNT: usize = 1;
const APP_SHORT_NAME: &'static str = "cube";
const APP_LONG_NAME: &'static str = "The Vulkan Cube Demo Program";

fn main() {
    let mut demo = Demo::init(std::env::args());

    demo.prepare();
    demo.run();

    mem::forget(demo); // FIXME: shouldn't need this, eventually
}


fn vkassert(result: VkResult) {
    assert_eq!(result, Enum_VkResult::VK_SUCCESS);
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


struct Demo<'a> {
    cfg: DemoConfig,
    prepared: bool,
    projection_matrix: Matrix4<f32>,
    view_matrix: Matrix4<f32>,
    model_matrix: Matrix4<f32>,
    spin_angle: f32,
    pause: bool,
    quit: bool,
    curFrame: i32,
    spin_increment: f32,

    current_buffer: u32,

    windowing: DemoWindowing<'a>,
    vk: DemoVk,
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

        let windowing = DemoWindowing::new(&cfg);

        let vk = DemoVk::new(&cfg, &windowing);

        Demo {
            cfg: cfg,
            windowing: windowing,
            vk: vk,
            projection_matrix: perspective(Deg::new(45f32), 1.0, 0.1, 100.0),
            view_matrix: Matrix4::look_at(eye, origin, up),
            model_matrix: Matrix4::identity(),
            curFrame: 0,
            prepared: false,
            current_buffer: 0,
            pause: false,
            spin_angle: 0.01,
            spin_increment: 0.01,
            quit: false,
        }
    }

    fn prepare(&mut self) {
        unimplemented!();
    }

    fn run(&mut self) {
        unimplemented!();
    }
}

impl<'a> Drop for Demo<'a> {
    fn drop(&mut self) {
        // TODO: cleanup
    }
}


struct DemoConfig {
    use_staging_buffer: bool,
    use_break: bool,
    validate: bool,
    frameCount: i32,
    width: u16,
    height: u16,
}

impl DemoConfig {
    fn new() -> Self { DemoConfig {
        use_staging_buffer: false,
        use_break: false,
        validate: false,
        frameCount: std::i32::MAX,
        width: 500,
        height: 500,
    } }
}


struct DemoVk {
    demo_swapchain: DemoVkSwapchain,
    demo_vk_base:   DemoVkBase,
}

impl DemoVk {
    fn new(cfg: &DemoConfig, windowing: &DemoWindowing) -> Self {
        let mut base = DemoVkBase::new(cfg);
        let sc = DemoVkSwapchain::new(cfg, &mut base, windowing);

        DemoVk {
            demo_vk_base: base,
            demo_swapchain: sc,
        }
    }
}


struct DemoVkBase {
    inst:                     VkInstance,
    gpu:                      VkPhysicalDevice,
    gpu_props:                VkPhysicalDeviceProperties,
    queue_props:              Vec<VkQueueFamilyProperties>, // queue_count
    extension_names:          Vec<&'static str>,
    device_validation_layers: Vec<&'static str>,
    fns:                      DemoFns,
}

impl DemoVkBase {
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
            let mut inst: VkInstance = mem::uninitialized();
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
            unsafe { mem::uninitialized() };
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
        let mut physDevFeatures: VkPhysicalDeviceFeatures = unsafe { mem::uninitialized() };
        unsafe {
            vkGetPhysicalDeviceFeatures(gpu, &mut physDevFeatures);
        }

        let gipa = vkGetInstanceProcAddr;
        let fns = DemoFns {
            fpGetPhysicalDeviceSurfaceSupportKHR     : unsafe { mem::transmute::<_, PFN_vkGetPhysicalDeviceSurfaceSupportKHR     >(gipa(inst, str_to_ptr("fpGetPhysicalDeviceSurfaceSupportKHR"     ))) },
            fpGetPhysicalDeviceSurfaceCapabilitiesKHR: unsafe { mem::transmute::<_, PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR>(gipa(inst, str_to_ptr("fpGetPhysicalDeviceSurfaceCapabilitiesKHR"))) },
            fpGetPhysicalDeviceSurfaceFormatsKHR     : unsafe { mem::transmute::<_, PFN_vkGetPhysicalDeviceSurfaceFormatsKHR     >(gipa(inst, str_to_ptr("fpGetPhysicalDeviceSurfaceFormatsKHR"     ))) },
            fpGetPhysicalDeviceSurfacePresentModesKHR: unsafe { mem::transmute::<_, PFN_vkGetPhysicalDeviceSurfacePresentModesKHR>(gipa(inst, str_to_ptr("fpGetPhysicalDeviceSurfacePresentModesKHR"))) },
            fpGetSwapchainImagesKHR                  : unsafe { mem::transmute::<_, PFN_vkGetSwapchainImagesKHR                  >(gipa(inst, str_to_ptr("fpGetSwapchainImagesKHR"                  ))) },

            fpCreateSwapchainKHR:       None,
            fpDestroySwapchainKHR:      None,
            fpAcquireNextImageKHR:      None,
            fpQueuePresentKHR:          None,

            CreateDebugReportCallback:  None,
            DestroyDebugReportCallback: None,
            msg_callback:               null_mut(),
            DebugReportMessage:         None,
        };

        DemoVkBase {
            inst: inst,
            gpu: gpu,
            gpu_props: gpu_props,
            queue_props: queue_props,
            extension_names: extension_names,
            device_validation_layers: device_validation_layers,
            fns: fns,
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


struct DemoFns {
    fpGetPhysicalDeviceSurfaceSupportKHR:      PFN_vkGetPhysicalDeviceSurfaceSupportKHR,
    fpGetPhysicalDeviceSurfaceCapabilitiesKHR: PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR,
    fpGetPhysicalDeviceSurfaceFormatsKHR:      PFN_vkGetPhysicalDeviceSurfaceFormatsKHR,
    fpGetPhysicalDeviceSurfacePresentModesKHR: PFN_vkGetPhysicalDeviceSurfacePresentModesKHR,
    fpGetSwapchainImagesKHR:                   PFN_vkGetSwapchainImagesKHR,

    fpCreateSwapchainKHR:                      PFN_vkCreateSwapchainKHR,
    fpDestroySwapchainKHR:                     PFN_vkDestroySwapchainKHR,
    fpAcquireNextImageKHR:                     PFN_vkAcquireNextImageKHR,
    fpQueuePresentKHR:                         PFN_vkQueuePresentKHR,

    CreateDebugReportCallback:                 PFN_vkCreateDebugReportCallbackEXT,
    DestroyDebugReportCallback:                PFN_vkDestroyDebugReportCallbackEXT,
    msg_callback:                              VkDebugReportCallbackEXT,
    DebugReportMessage:                        PFN_vkDebugReportMessageEXT,
}


struct DemoVkSwapchain {
    surface:                   VkSurfaceKHR,
    device:                    VkDevice,
    queue:                     VkQueue,
    memory_properties:         VkPhysicalDeviceMemoryProperties,
    color_space:               VkColorSpaceKHR,

    graphics_queue_node_index: u32,
    format:                    VkFormat,
    demo_swapchain:            Option<DemoVkSwapchainPrepared>,
}

impl DemoVkSwapchain {
    fn new(cfg: &DemoConfig,
           base: &mut DemoVkBase,
           windowing: &DemoWindowing) -> Self {
        let createInfo = VkXcbSurfaceCreateInfoKHR {
            sType: Enum_VkStructureType::VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR,
            pNext: null(),
            flags: 0,
            connection: unsafe { windowing.connection.get_raw_conn() },
            window: windowing.window,
        };

        let surface = unsafe {
            let mut surface = mem::uninitialized();
            vkassert(vkCreateXcbSurfaceKHR(base.inst, &createInfo, null(), &mut surface));
            surface
        };

        // Iterate over each queue to learn whether it supports presenting:
        let mut supports_present = vec![0u32; base.queue_props.len()];
        for (i, sp) in supports_present.iter_mut().enumerate() {
            unsafe {
                base.fns.fpGetPhysicalDeviceSurfaceSupportKHR.unwrap()(
                    base.gpu, i as u32, surface, sp);
            }
        }

        let mut graphicsQueueNodeIndex = None;
        let mut presentQueueNodeIndex = None;
        // Search for a graphics and a present queue in the array of queue
        // families, try to find one that supports both
        for (i, qp) in base.queue_props.iter().enumerate() {
            if qp.queueFlags & (Enum_VkQueueFlagBits::VK_QUEUE_GRAPHICS_BIT as u32) != 0 {
                if let None = graphicsQueueNodeIndex {
                    graphicsQueueNodeIndex = Some(i as u32);
                }
                if supports_present[i] == 1 {
                    graphicsQueueNodeIndex = Some(i as u32);
                    presentQueueNodeIndex = Some(i as u32);
                    break;
                }
            }
        }
        if let None = presentQueueNodeIndex {
            // If didn't find a queue that supports both graphics and present,
            // then find a separate present queue.
            for i in 0..base.queue_props.len() {
                if supports_present[i] == 1 {
                    presentQueueNodeIndex = Some(i as u32);
                    break;
                }
            }
        }
        // Generate error if could not find both a graphics and a present queue
        let graphicsQueueNodeIndex = graphicsQueueNodeIndex.unwrap();
        let presentQueueNodeIndex  = presentQueueNodeIndex .unwrap();

        // TODO: Add support for separate queues, including presentation,
        //       synchronization, and appropriate tracking for QueueSubmit.
        // NOTE: While it is possible for an application to use a separate
        //       graphics and a present queues, this demo program assumes it is
        //       only using one:
        if graphicsQueueNodeIndex != presentQueueNodeIndex {
            panic!("Could not find a common graphics-and-present queue");
        }

        let device = create_device(cfg, base, graphicsQueueNodeIndex);

        let gdpa = unsafe { mem::transmute::<_, PFN_vkGetDeviceProcAddr>(vkGetInstanceProcAddr(base.inst, str_to_ptr("vkGetDeviceProcAddr"))) }.unwrap();

        base.fns.fpCreateSwapchainKHR    = unsafe { mem::transmute::<_, PFN_vkCreateSwapchainKHR   >(gdpa(device, str_to_ptr("vkCreateSwapchainKHR"   ))) };
        base.fns.fpDestroySwapchainKHR   = unsafe { mem::transmute::<_, PFN_vkDestroySwapchainKHR  >(gdpa(device, str_to_ptr("vkDestroySwapchainKHR"  ))) };
        base.fns.fpGetSwapchainImagesKHR = unsafe { mem::transmute::<_, PFN_vkGetSwapchainImagesKHR>(gdpa(device, str_to_ptr("vkGetSwapchainImagesKHR"))) };
        base.fns.fpAcquireNextImageKHR   = unsafe { mem::transmute::<_, PFN_vkAcquireNextImageKHR  >(gdpa(device, str_to_ptr("vkAcquireNextImageKHR"  ))) };
        base.fns.fpQueuePresentKHR       = unsafe { mem::transmute::<_, PFN_vkQueuePresentKHR      >(gdpa(device, str_to_ptr("vkQueuePresentKHR"      ))) };

        let mut queue = unsafe { mem::uninitialized() };
        unsafe { vkGetDeviceQueue(device, graphicsQueueNodeIndex, 0, &mut queue) };

        // Get the lits of `VkFormat`s that are supported:
        let mut formatCount = 0u32;
        unsafe {
            vkassert(base.fns.fpGetPhysicalDeviceSurfaceFormatsKHR.unwrap()(base.gpu, surface, &mut formatCount, null_mut()));
        }

        let mut surfFormats = Vec::with_capacity(formatCount as usize);
        unsafe {
            vkassert(base.fns.fpGetPhysicalDeviceSurfaceFormatsKHR.unwrap()(base.gpu, surface, &mut formatCount, surfFormats.as_mut_ptr()));
            surfFormats.set_len(formatCount as usize);
        }

        // If the format list includes just one entry of VK_FORMAT_UNDEFINED,
        // the surface has no preferred format. Otherwise, at least one
        // supported format will be returned.
        let format =
            if formatCount == 1 &&
                surfFormats[0].format == Enum_VkFormat::VK_FORMAT_UNDEFINED {
                Enum_VkFormat::VK_FORMAT_B8G8R8A8_UNORM
            } else {
                assert!(formatCount >= 1);
                surfFormats[0].format
            };
        let color_space = surfFormats[0].colorSpace;

        let memory_properties = unsafe {
            let mut mp = mem::uninitialized();
            vkGetPhysicalDeviceMemoryProperties(base.gpu, &mut mp);
            mp
        };

        DemoVkSwapchain {
            surface: surface,
            device: device,
            queue: queue,
            memory_properties: memory_properties,
            color_space: color_space,
            graphics_queue_node_index: graphicsQueueNodeIndex,
            format: format,
            demo_swapchain: None,
        }
    }
}

fn create_device(cfg: &DemoConfig, base: &DemoVkBase, graphics_queue_node_index: u32) -> VkDevice {
    let queue_priorities = [0.0f32];
    let queue = VkDeviceQueueCreateInfo {
        sType: Enum_VkStructureType::VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
        pNext: null(),
        queueFamilyIndex: graphics_queue_node_index,
        queueCount: 1,
        pQueuePriorities: &queue_priorities as *const f32,
        flags: 0,
    };

    let device = VkDeviceCreateInfo {
        sType: Enum_VkStructureType::VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
        pNext: null(),
        queueCreateInfoCount: 1,
        pQueueCreateInfos: &queue,
        enabledLayerCount:
            if cfg.validate {
                base.device_validation_layers.len() as u32
            } else {
                0
            },
        ppEnabledLayerNames:
            if cfg.validate {
                vec_str_to_vec_ptr(&base.device_validation_layers).as_ptr()
            } else {
                null()
            },
            enabledExtensionCount: base.extension_names.len() as u32,
            ppEnabledExtensionNames: vec_str_to_vec_ptr(&base.extension_names).as_ptr(),
            // If specific features are required, pass them in here:
            pEnabledFeatures: null(),
            flags: 0,
    };

    unsafe {
        let mut ret = mem::uninitialized();
        vkassert(vkCreateDevice(base.gpu, &device, null(), &mut ret));
        ret
    }
}


struct DemoVkSwapchainPrepared {
    swapchain:                 VkSwapchainKHR,
    buffers:                   Vec<SwapchainBuffers>, // swapchainImageCount
    cmd_pool:                  VkCommandPool,
    depth:                     Depth,
    textures:                  [TextureObject; DEMO_TEXTURE_COUNT],
    uniform_data:              UniformData,

    cmd:                       VkCommandBuffer,
    pipeline_layout:           VkPipelineLayout,
    desc_layout:               VkDescriptorSetLayout,
    pipelineCache:             VkPipelineCache,
    render_pass:               VkRenderPass,
    pipeline:                  VkPipeline,

    vert_shader_module:        VkShaderModule,
    frag_shader_module:        VkShaderModule,

    desc_pool:                 VkDescriptorPool,
    desc_set:                  VkDescriptorSet,

    framebuffers:              Vec<VkFramebuffer>,
}


struct DemoWindowing<'a> {
    connection:            xcb::base::Connection,
    screen:                xcb::xproto::Screen<'a>,
    window:                xcb::xproto::Window,
    atom_wm_delete_window: xcb::xproto::InternAtomReply,
}

impl<'a> DemoWindowing<'a> {
    fn new(cfg: &DemoConfig) -> Self {
        use xcb::base::*;
        use xcb::xproto::*;
        let (connection, screen_num) = Connection::connect();
        let screen = {
            let setup = connection.get_setup();
            let screen = setup.roots().nth(screen_num as usize).unwrap();
            unsafe {
                // Ignore the lifetimes on screen. I don't know why I have to
                // do this, precisely, but I'm having trouble with the lifetime
                // bounds in rust-xcb.
                mem::transmute::<Screen, Screen>(screen)
            }
        };

        let window = connection.generate_id();
        let value_mask = CW_BACK_PIXEL | CW_EVENT_MASK;
        let value_list = vec![
            (screen.black_pixel(), EVENT_MASK_KEY_RELEASE | EVENT_MASK_EXPOSURE | EVENT_MASK_STRUCTURE_NOTIFY),
        ];

        create_window(&connection,
                      COPY_FROM_PARENT as u8,
                      window, screen.root(),
                      0, 0, cfg.width, cfg.height, 0,
                      WINDOW_CLASS_INPUT_OUTPUT as u16,
                      screen.root_visual(),
                      &value_list);

        // Magic code that will send notification when window is destroyed
        let cookie = intern_atom(&connection, true, "WM_PROTOCOLS");
        let reply = cookie.get_reply().unwrap();

        let cookie2 = intern_atom(&connection, false, "WM_DELETE_WINDOW");
        let atom_wm_delete_window = cookie2.get_reply().unwrap();

        change_property(&connection,
                        PROP_MODE_REPLACE as u8,
                        window,
                        reply.atom(), 4, 32,
                        unsafe { mem::transmute::<&u32, &[u8; 4]>(&atom_wm_delete_window.atom()) });

        map_window(&connection, window);

        // Force the x/y coordinates to 100,100 results are identical in
        // consecutive runs
        let coords = [(CONFIG_WINDOW_X as u16, 100), (CONFIG_WINDOW_Y as u16, 100)];
        configure_window(&connection, window, &coords);

        DemoWindowing {
            connection: connection,
            screen: screen,
            window: window,
            atom_wm_delete_window: atom_wm_delete_window,
        }
    }
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

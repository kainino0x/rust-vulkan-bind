use vk::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ApplicationInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub pApplicationName: *const ::std::os::raw::c_char,
    pub applicationVersion: u32,
    pub pEngineName: *const ::std::os::raw::c_char,
    pub engineVersion: u32,
    pub apiVersion: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct InstanceCreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: InstanceCreateFlags,
    pub pApplicationInfo: *const ApplicationInfo,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: *const *const ::std::os::raw::c_char,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: *const *const ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AllocationCallbacks {
    pub pUserData: *mut ::std::os::raw::c_void,
    pub pfnAllocation: PFN_vkAllocationFunction,
    pub pfnReallocation: PFN_vkReallocationFunction,
    pub pfnFree: PFN_vkFreeFunction,
    pub pfnInternalAllocation: PFN_vkInternalAllocationNotification,
    pub pfnInternalFree: PFN_vkInternalFreeNotification,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFeatures {
    pub robustBufferAccess: Bool32,
    pub fullDrawIndexUint32: Bool32,
    pub imageCubeArray: Bool32,
    pub independentBlend: Bool32,
    pub geometryShader: Bool32,
    pub tessellationShader: Bool32,
    pub sampleRateShading: Bool32,
    pub dualSrcBlend: Bool32,
    pub logicOp: Bool32,
    pub multiDrawIndirect: Bool32,
    pub drawIndirectFirstInstance: Bool32,
    pub depthClamp: Bool32,
    pub depthBiasClamp: Bool32,
    pub fillModeNonSolid: Bool32,
    pub depthBounds: Bool32,
    pub wideLines: Bool32,
    pub largePoints: Bool32,
    pub alphaToOne: Bool32,
    pub multiViewport: Bool32,
    pub samplerAnisotropy: Bool32,
    pub textureCompressionETC2: Bool32,
    pub textureCompressionASTC_LDR: Bool32,
    pub textureCompressionBC: Bool32,
    pub occlusionQueryPrecise: Bool32,
    pub pipelineStatisticsQuery: Bool32,
    pub vertexPipelineStoresAndAtomics: Bool32,
    pub fragmentStoresAndAtomics: Bool32,
    pub shaderTessellationAndGeometryPointSize: Bool32,
    pub shaderImageGatherExtended: Bool32,
    pub shaderStorageImageExtendedFormats: Bool32,
    pub shaderStorageImageMultisample: Bool32,
    pub shaderStorageImageReadWithoutFormat: Bool32,
    pub shaderStorageImageWriteWithoutFormat: Bool32,
    pub shaderUniformBufferArrayDynamicIndexing: Bool32,
    pub shaderSampledImageArrayDynamicIndexing: Bool32,
    pub shaderStorageBufferArrayDynamicIndexing: Bool32,
    pub shaderStorageImageArrayDynamicIndexing: Bool32,
    pub shaderClipDistance: Bool32,
    pub shaderCullDistance: Bool32,
    pub shaderFloat64: Bool32,
    pub shaderInt64: Bool32,
    pub shaderInt16: Bool32,
    pub shaderResourceResidency: Bool32,
    pub shaderResourceMinLod: Bool32,
    pub sparseBinding: Bool32,
    pub sparseResidencyBuffer: Bool32,
    pub sparseResidencyImage2D: Bool32,
    pub sparseResidencyImage3D: Bool32,
    pub sparseResidency2Samples: Bool32,
    pub sparseResidency4Samples: Bool32,
    pub sparseResidency8Samples: Bool32,
    pub sparseResidency16Samples: Bool32,
    pub sparseResidencyAliased: Bool32,
    pub variableMultisampleRate: Bool32,
    pub inheritedQueries: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FormatProperties {
    pub linearTilingFeatures: FormatFeatureFlags,
    pub optimalTilingFeatures: FormatFeatureFlags,
    pub bufferFeatures: FormatFeatureFlags,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Extent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct ImageFormatProperties {
    pub maxExtent: Extent3D,
    pub maxMipLevels: u32,
    pub maxArrayLayers: u32,
    pub sampleCounts: SampleCountFlags,
    pub maxResourceSize: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceLimits {
    pub maxImageDimension1D: u32,
    pub maxImageDimension2D: u32,
    pub maxImageDimension3D: u32,
    pub maxImageDimensionCube: u32,
    pub maxImageArrayLayers: u32,
    pub maxTexelBufferElements: u32,
    pub maxUniformBufferRange: u32,
    pub maxStorageBufferRange: u32,
    pub maxPushConstantsSize: u32,
    pub maxMemoryAllocationCount: u32,
    pub maxSamplerAllocationCount: u32,
    pub bufferImageGranularity: DeviceSize,
    pub sparseAddressSpaceSize: DeviceSize,
    pub maxBoundDescriptorSets: u32,
    pub maxPerStageDescriptorSamplers: u32,
    pub maxPerStageDescriptorUniformBuffers: u32,
    pub maxPerStageDescriptorStorageBuffers: u32,
    pub maxPerStageDescriptorSampledImages: u32,
    pub maxPerStageDescriptorStorageImages: u32,
    pub maxPerStageDescriptorInputAttachments: u32,
    pub maxPerStageResources: u32,
    pub maxDescriptorSetSamplers: u32,
    pub maxDescriptorSetUniformBuffers: u32,
    pub maxDescriptorSetUniformBuffersDynamic: u32,
    pub maxDescriptorSetStorageBuffers: u32,
    pub maxDescriptorSetStorageBuffersDynamic: u32,
    pub maxDescriptorSetSampledImages: u32,
    pub maxDescriptorSetStorageImages: u32,
    pub maxDescriptorSetInputAttachments: u32,
    pub maxVertexInputAttributes: u32,
    pub maxVertexInputBindings: u32,
    pub maxVertexInputAttributeOffset: u32,
    pub maxVertexInputBindingStride: u32,
    pub maxVertexOutputComponents: u32,
    pub maxTessellationGenerationLevel: u32,
    pub maxTessellationPatchSize: u32,
    pub maxTessellationControlPerVertexInputComponents: u32,
    pub maxTessellationControlPerVertexOutputComponents: u32,
    pub maxTessellationControlPerPatchOutputComponents: u32,
    pub maxTessellationControlTotalOutputComponents: u32,
    pub maxTessellationEvaluationInputComponents: u32,
    pub maxTessellationEvaluationOutputComponents: u32,
    pub maxGeometryShaderInvocations: u32,
    pub maxGeometryInputComponents: u32,
    pub maxGeometryOutputComponents: u32,
    pub maxGeometryOutputVertices: u32,
    pub maxGeometryTotalOutputComponents: u32,
    pub maxFragmentInputComponents: u32,
    pub maxFragmentOutputAttachments: u32,
    pub maxFragmentDualSrcAttachments: u32,
    pub maxFragmentCombinedOutputResources: u32,
    pub maxComputeSharedMemorySize: u32,
    pub maxComputeWorkGroupCount: [u32; 3usize],
    pub maxComputeWorkGroupInvocations: u32,
    pub maxComputeWorkGroupSize: [u32; 3usize],
    pub subPixelPrecisionBits: u32,
    pub subTexelPrecisionBits: u32,
    pub mipmapPrecisionBits: u32,
    pub maxDrawIndexedIndexValue: u32,
    pub maxDrawIndirectCount: u32,
    pub maxSamplerLodBias: ::std::os::raw::c_float,
    pub maxSamplerAnisotropy: ::std::os::raw::c_float,
    pub maxViewports: u32,
    pub maxViewportDimensions: [u32; 2usize],
    pub viewportBoundsRange: [::std::os::raw::c_float; 2usize],
    pub viewportSubPixelBits: u32,
    pub minMemoryMapAlignment: usize,
    pub minTexelBufferOffsetAlignment: DeviceSize,
    pub minUniformBufferOffsetAlignment: DeviceSize,
    pub minStorageBufferOffsetAlignment: DeviceSize,
    pub minTexelOffset: i32,
    pub maxTexelOffset: u32,
    pub minTexelGatherOffset: i32,
    pub maxTexelGatherOffset: u32,
    pub minInterpolationOffset: ::std::os::raw::c_float,
    pub maxInterpolationOffset: ::std::os::raw::c_float,
    pub subPixelInterpolationOffsetBits: u32,
    pub maxFramebufferWidth: u32,
    pub maxFramebufferHeight: u32,
    pub maxFramebufferLayers: u32,
    pub framebufferColorSampleCounts: SampleCountFlags,
    pub framebufferDepthSampleCounts: SampleCountFlags,
    pub framebufferStencilSampleCounts: SampleCountFlags,
    pub framebufferNoAttachmentsSampleCounts: SampleCountFlags,
    pub maxColorAttachments: u32,
    pub sampledImageColorSampleCounts: SampleCountFlags,
    pub sampledImageIntegerSampleCounts: SampleCountFlags,
    pub sampledImageDepthSampleCounts: SampleCountFlags,
    pub sampledImageStencilSampleCounts: SampleCountFlags,
    pub storageImageSampleCounts: SampleCountFlags,
    pub maxSampleMaskWords: u32,
    pub timestampComputeAndGraphics: Bool32,
    pub timestampPeriod: ::std::os::raw::c_float,
    pub maxClipDistances: u32,
    pub maxCullDistances: u32,
    pub maxCombinedClipAndCullDistances: u32,
    pub discreteQueuePriorities: u32,
    pub pointSizeRange: [::std::os::raw::c_float; 2usize],
    pub lineWidthRange: [::std::os::raw::c_float; 2usize],
    pub pointSizeGranularity: ::std::os::raw::c_float,
    pub lineWidthGranularity: ::std::os::raw::c_float,
    pub strictLines: Bool32,
    pub standardSampleLocations: Bool32,
    pub optimalBufferCopyOffsetAlignment: DeviceSize,
    pub optimalBufferCopyRowPitchAlignment: DeviceSize,
    pub nonCoherentAtomSize: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceSparseProperties {
    pub residencyStandard2DBlockShape: Bool32,
    pub residencyStandard2DMultisampleBlockShape: Bool32,
    pub residencyStandard3DBlockShape: Bool32,
    pub residencyAlignedMipSize: Bool32,
    pub residencyNonResidentStrict: Bool32,
}
#[repr(C)]
#[derive(Copy)]
pub struct PhysicalDeviceProperties {
    pub apiVersion: u32,
    pub driverVersion: u32,
    pub vendorID: u32,
    pub deviceID: u32,
    pub deviceType: PhysicalDeviceType,
    pub deviceName: [::std::os::raw::c_char; 256usize],
    pub pipelineCacheUUID: [u8; 16usize],
    pub limits: PhysicalDeviceLimits,
    pub sparseProperties: PhysicalDeviceSparseProperties,
}
impl Clone for PhysicalDeviceProperties {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct QueueFamilyProperties {
    pub queueFlags: QueueFlags,
    pub queueCount: u32,
    pub timestampValidBits: u32,
    pub minImageTransferGranularity: Extent3D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryType {
    pub propertyFlags: MemoryPropertyFlags,
    pub heapIndex: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryHeap {
    pub size: DeviceSize,
    pub flags: MemoryHeapFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMemoryProperties {
    pub memoryTypeCount: u32,
    pub memoryTypes: [MemoryType; 32usize],
    pub memoryHeapCount: u32,
    pub memoryHeaps: [MemoryHeap; 16usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceQueueCreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: DeviceQueueCreateFlags,
    pub queueFamilyIndex: u32,
    pub queueCount: u32,
    pub pQueuePriorities: *const ::std::os::raw::c_float,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceCreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: DeviceCreateFlags,
    pub queueCreateInfoCount: u32,
    pub pQueueCreateInfos: *const DeviceQueueCreateInfo,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: *const *const ::std::os::raw::c_char,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: *const *const ::std::os::raw::c_char,
    pub pEnabledFeatures: *const PhysicalDeviceFeatures,
}
#[repr(C)]
#[derive(Copy)]
pub struct ExtensionProperties {
    pub extensionName: [::std::os::raw::c_char; 256usize],
    pub specVersion: u32,
}
impl Clone for ExtensionProperties {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Copy)]
pub struct LayerProperties {
    pub layerName: [::std::os::raw::c_char; 256usize],
    pub specVersion: u32,
    pub implementationVersion: u32,
    pub description: [::std::os::raw::c_char; 256usize],
}
impl Clone for LayerProperties {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SubmitInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphores: *const Semaphore,
    pub pWaitDstStageMask: *const PipelineStageFlags,
    pub commandBufferCount: u32,
    pub pCommandBuffers: *const CommandBuffer,
    pub signalSemaphoreCount: u32,
    pub pSignalSemaphores: *const Semaphore,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryAllocateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub allocationSize: DeviceSize,
    pub memoryTypeIndex: u32,
}
#[repr(C)]
pub struct MappedMemoryRange {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub memory: DeviceMemory,
    pub offset: DeviceSize,
    pub size: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryRequirements {
    pub size: DeviceSize,
    pub alignment: DeviceSize,
    pub memoryTypeBits: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SparseImageFormatProperties {
    pub aspectMask: ImageAspectFlags,
    pub imageGranularity: Extent3D,
    pub flags: SparseImageFormatFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SparseImageMemoryRequirements {
    pub formatProperties: SparseImageFormatProperties,
    pub imageMipTailFirstLod: u32,
    pub imageMipTailSize: DeviceSize,
    pub imageMipTailOffset: DeviceSize,
    pub imageMipTailStride: DeviceSize,
}
#[repr(C)]
pub struct SparseMemoryBind {
    pub resourceOffset: DeviceSize,
    pub size: DeviceSize,
    pub memory: DeviceMemory,
    pub memoryOffset: DeviceSize,
    pub flags: SparseMemoryBindFlags,
}
#[repr(C)]
pub struct SparseBufferMemoryBindInfo {
    pub buffer: Buffer,
    pub bindCount: u32,
    pub pBinds: *const SparseMemoryBind,
}
#[repr(C)]
pub struct SparseImageOpaqueMemoryBindInfo {
    pub image: Image,
    pub bindCount: u32,
    pub pBinds: *const SparseMemoryBind,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct ImageSubresource {
    pub aspectMask: ImageAspectFlags,
    pub mipLevel: u32,
    pub arrayLayer: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Offset3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
#[repr(C)]
pub struct SparseImageMemoryBind {
    pub subresource: ImageSubresource,
    pub offset: Offset3D,
    pub extent: Extent3D,
    pub memory: DeviceMemory,
    pub memoryOffset: DeviceSize,
    pub flags: SparseMemoryBindFlags,
}
#[repr(C)]
pub struct SparseImageMemoryBindInfo {
    pub image: Image,
    pub bindCount: u32,
    pub pBinds: *const SparseImageMemoryBind,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindSparseInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphores: *const Semaphore,
    pub bufferBindCount: u32,
    pub pBufferBinds: *const SparseBufferMemoryBindInfo,
    pub imageOpaqueBindCount: u32,
    pub pImageOpaqueBinds: *const SparseImageOpaqueMemoryBindInfo,
    pub imageBindCount: u32,
    pub pImageBinds: *const SparseImageMemoryBindInfo,
    pub signalSemaphoreCount: u32,
    pub pSignalSemaphores: *const Semaphore,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FenceCreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: FenceCreateFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SemaphoreCreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: SemaphoreCreateFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct EventCreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: EventCreateFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct QueryPoolCreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: QueryPoolCreateFlags,
    pub queryType: QueryType,
    pub queryCount: u32,
    pub pipelineStatistics: QueryPipelineStatisticFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferCreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: BufferCreateFlags,
    pub size: DeviceSize,
    pub usage: BufferUsageFlags,
    pub sharingMode: SharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
}
#[repr(C)]
pub struct BufferViewCreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: BufferViewCreateFlags,
    pub buffer: Buffer,
    pub format: Format,
    pub offset: DeviceSize,
    pub range: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageCreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: ImageCreateFlags,
    pub imageType: ImageType,
    pub format: Format,
    pub extent: Extent3D,
    pub mipLevels: u32,
    pub arrayLayers: u32,
    pub samples: SampleCountFlags,
    pub tiling: ImageTiling,
    pub usage: ImageUsageFlags,
    pub sharingMode: SharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
    pub initialLayout: ImageLayout,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SubresourceLayout {
    pub offset: DeviceSize,
    pub size: DeviceSize,
    pub rowPitch: DeviceSize,
    pub arrayPitch: DeviceSize,
    pub depthPitch: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct ComponentMapping {
    pub r: ComponentSwizzle,
    pub g: ComponentSwizzle,
    pub b: ComponentSwizzle,
    pub a: ComponentSwizzle,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct ImageSubresourceRange {
    pub aspectMask: ImageAspectFlags,
    pub baseMipLevel: u32,
    pub levelCount: u32,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}
#[repr(C)]
pub struct ImageViewCreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: ImageViewCreateFlags,
    pub image: Image,
    pub viewType: ImageViewType,
    pub format: Format,
    pub components: ComponentMapping,
    pub subresourceRange: ImageSubresourceRange,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShaderModuleCreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: ShaderModuleCreateFlags,
    pub codeSize: usize,
    pub pCode: *const u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineCacheCreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: PipelineCacheCreateFlags,
    pub initialDataSize: usize,
    pub pInitialData: *const ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SpecializationMapEntry {
    pub constantID: u32,
    pub offset: u32,
    pub size: usize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SpecializationInfo {
    pub mapEntryCount: u32,
    pub pMapEntries: *const SpecializationMapEntry,
    pub dataSize: usize,
    pub pData: *const ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineShaderStageCreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: PipelineShaderStageCreateFlags,
    pub stage: ShaderStageFlags,
    pub module: ShaderModule,
    pub pName: *const ::std::os::raw::c_char,
    pub pSpecializationInfo: *const SpecializationInfo,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VertexInputBindingDescription {
    pub binding: u32,
    pub stride: u32,
    pub inputRate: VertexInputRate,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VertexInputAttributeDescription {
    pub location: u32,
    pub binding: u32,
    pub format: Format,
    pub offset: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineVertexInputStateCreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: PipelineVertexInputStateCreateFlags,
    pub vertexBindingDescriptionCount: u32,
    pub pVertexBindingDescriptions: *const VertexInputBindingDescription,
    pub vertexAttributeDescriptionCount: u32,
    pub pVertexAttributeDescriptions: *const VertexInputAttributeDescription,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineInputAssemblyStateCreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: PipelineInputAssemblyStateCreateFlags,
    pub topology: PrimitiveTopology,
    pub primitiveRestartEnable: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineTessellationStateCreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: PipelineTessellationStateCreateFlags,
    pub patchControlPoints: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Viewport {
    pub x: ::std::os::raw::c_float,
    pub y: ::std::os::raw::c_float,
    pub width: ::std::os::raw::c_float,
    pub height: ::std::os::raw::c_float,
    pub minDepth: ::std::os::raw::c_float,
    pub maxDepth: ::std::os::raw::c_float,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Offset2D {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Extent2D {
    pub width: u32,
    pub height: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Rect2D {
    pub offset: Offset2D,
    pub extent: Extent2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineViewportStateCreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: PipelineViewportStateCreateFlags,
    pub viewportCount: u32,
    pub pViewports: *const Viewport,
    pub scissorCount: u32,
    pub pScissors: *const Rect2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineRasterizationStateCreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: PipelineRasterizationStateCreateFlags,
    pub depthClampEnable: Bool32,
    pub rasterizerDiscardEnable: Bool32,
    pub polygonMode: PolygonMode,
    pub cullMode: CullModeFlags,
    pub frontFace: FrontFace,
    pub depthBiasEnable: Bool32,
    pub depthBiasConstantFactor: ::std::os::raw::c_float,
    pub depthBiasClamp: ::std::os::raw::c_float,
    pub depthBiasSlopeFactor: ::std::os::raw::c_float,
    pub lineWidth: ::std::os::raw::c_float,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineMultisampleStateCreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: PipelineMultisampleStateCreateFlags,
    pub rasterizationSamples: SampleCountFlags,
    pub sampleShadingEnable: Bool32,
    pub minSampleShading: ::std::os::raw::c_float,
    pub pSampleMask: *const SampleMask,
    pub alphaToCoverageEnable: Bool32,
    pub alphaToOneEnable: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct StencilOpState {
    pub failOp: StencilOp,
    pub passOp: StencilOp,
    pub depthFailOp: StencilOp,
    pub compareOp: CompareOp,
    pub compareMask: u32,
    pub writeMask: u32,
    pub reference: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineDepthStencilStateCreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: PipelineDepthStencilStateCreateFlags,
    pub depthTestEnable: Bool32,
    pub depthWriteEnable: Bool32,
    pub depthCompareOp: CompareOp,
    pub depthBoundsTestEnable: Bool32,
    pub stencilTestEnable: Bool32,
    pub front: StencilOpState,
    pub back: StencilOpState,
    pub minDepthBounds: ::std::os::raw::c_float,
    pub maxDepthBounds: ::std::os::raw::c_float,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineColorBlendAttachmentState {
    pub blendEnable: Bool32,
    pub srcColorBlendFactor: BlendFactor,
    pub dstColorBlendFactor: BlendFactor,
    pub colorBlendOp: BlendOp,
    pub srcAlphaBlendFactor: BlendFactor,
    pub dstAlphaBlendFactor: BlendFactor,
    pub alphaBlendOp: BlendOp,
    pub colorWriteMask: ColorComponentFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineColorBlendStateCreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: PipelineColorBlendStateCreateFlags,
    pub logicOpEnable: Bool32,
    pub logicOp: LogicOp,
    pub attachmentCount: u32,
    pub pAttachments: *const PipelineColorBlendAttachmentState,
    pub blendConstants: [::std::os::raw::c_float; 4usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineDynamicStateCreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: PipelineDynamicStateCreateFlags,
    pub dynamicStateCount: u32,
    pub pDynamicStates: *const DynamicState,
}
#[repr(C)]
pub struct GraphicsPipelineCreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: PipelineCreateFlags,
    pub stageCount: u32,
    pub pStages: *const PipelineShaderStageCreateInfo,
    pub pVertexInputState: *const PipelineVertexInputStateCreateInfo,
    pub pInputAssemblyState: *const PipelineInputAssemblyStateCreateInfo,
    pub pTessellationState: *const PipelineTessellationStateCreateInfo,
    pub pViewportState: *const PipelineViewportStateCreateInfo,
    pub pRasterizationState: *const PipelineRasterizationStateCreateInfo,
    pub pMultisampleState: *const PipelineMultisampleStateCreateInfo,
    pub pDepthStencilState: *const PipelineDepthStencilStateCreateInfo,
    pub pColorBlendState: *const PipelineColorBlendStateCreateInfo,
    pub pDynamicState: *const PipelineDynamicStateCreateInfo,
    pub layout: PipelineLayout,
    pub renderPass: RenderPass,
    pub subpass: u32,
    pub basePipelineHandle: Pipeline,
    pub basePipelineIndex: i32,
}
#[repr(C)]
pub struct ComputePipelineCreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: PipelineCreateFlags,
    pub stage: PipelineShaderStageCreateInfo,
    pub layout: PipelineLayout,
    pub basePipelineHandle: Pipeline,
    pub basePipelineIndex: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PushConstantRange {
    pub stageFlags: ShaderStageFlags,
    pub offset: u32,
    pub size: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineLayoutCreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: PipelineLayoutCreateFlags,
    pub setLayoutCount: u32,
    pub pSetLayouts: *const DescriptorSetLayout,
    pub pushConstantRangeCount: u32,
    pub pPushConstantRanges: *const PushConstantRange,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SamplerCreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: SamplerCreateFlags,
    pub magFilter: Filter,
    pub minFilter: Filter,
    pub mipmapMode: SamplerMipmapMode,
    pub addressModeU: SamplerAddressMode,
    pub addressModeV: SamplerAddressMode,
    pub addressModeW: SamplerAddressMode,
    pub mipLodBias: ::std::os::raw::c_float,
    pub anisotropyEnable: Bool32,
    pub maxAnisotropy: ::std::os::raw::c_float,
    pub compareEnable: Bool32,
    pub compareOp: CompareOp,
    pub minLod: ::std::os::raw::c_float,
    pub maxLod: ::std::os::raw::c_float,
    pub borderColor: BorderColor,
    pub unnormalizedCoordinates: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorSetLayoutBinding {
    pub binding: u32,
    pub descriptorType: DescriptorType,
    pub descriptorCount: u32,
    pub stageFlags: ShaderStageFlags,
    pub pImmutableSamplers: *const Sampler,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorSetLayoutCreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: DescriptorSetLayoutCreateFlags,
    pub bindingCount: u32,
    pub pBindings: *const DescriptorSetLayoutBinding,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorPoolSize {
    pub typ: DescriptorType,
    pub descriptorCount: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorPoolCreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: DescriptorPoolCreateFlags,
    pub maxSets: u32,
    pub poolSizeCount: u32,
    pub pPoolSizes: *const DescriptorPoolSize,
}
#[repr(C)]
pub struct DescriptorSetAllocateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub descriptorPool: DescriptorPool,
    pub descriptorSetCount: u32,
    pub pSetLayouts: *const DescriptorSetLayout,
}
#[repr(C)]
pub struct DescriptorImageInfo {
    pub sampler: Sampler,
    pub imageView: ImageView,
    pub imageLayout: ImageLayout,
}
#[repr(C)]
pub struct DescriptorBufferInfo {
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub range: DeviceSize,
}
#[repr(C)]
pub struct WriteDescriptorSet {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub dstSet: DescriptorSet,
    pub dstBinding: u32,
    pub dstArrayElement: u32,
    pub descriptorCount: u32,
    pub descriptorType: DescriptorType,
    pub pImageInfo: *const DescriptorImageInfo,
    pub pBufferInfo: *const DescriptorBufferInfo,
    pub pTexelBufferView: *const BufferView,
}
#[repr(C)]
pub struct CopyDescriptorSet {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub srcSet: DescriptorSet,
    pub srcBinding: u32,
    pub srcArrayElement: u32,
    pub dstSet: DescriptorSet,
    pub dstBinding: u32,
    pub dstArrayElement: u32,
    pub descriptorCount: u32,
}
#[repr(C)]
pub struct FramebufferCreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: FramebufferCreateFlags,
    pub renderPass: RenderPass,
    pub attachmentCount: u32,
    pub pAttachments: *const ImageView,
    pub width: u32,
    pub height: u32,
    pub layers: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AttachmentDescription {
    pub flags: AttachmentDescriptionFlags,
    pub format: Format,
    pub samples: SampleCountFlags,
    pub loadOp: AttachmentLoadOp,
    pub storeOp: AttachmentStoreOp,
    pub stencilLoadOp: AttachmentLoadOp,
    pub stencilStoreOp: AttachmentStoreOp,
    pub initialLayout: ImageLayout,
    pub finalLayout: ImageLayout,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AttachmentReference {
    pub attachment: u32,
    pub layout: ImageLayout,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SubpassDescription {
    pub flags: SubpassDescriptionFlags,
    pub pipelineBindPoint: PipelineBindPoint,
    pub inputAttachmentCount: u32,
    pub pInputAttachments: *const AttachmentReference,
    pub colorAttachmentCount: u32,
    pub pColorAttachments: *const AttachmentReference,
    pub pResolveAttachments: *const AttachmentReference,
    pub pDepthStencilAttachment: *const AttachmentReference,
    pub preserveAttachmentCount: u32,
    pub pPreserveAttachments: *const u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SubpassDependency {
    pub srcSubpass: u32,
    pub dstSubpass: u32,
    pub srcStageMask: PipelineStageFlags,
    pub dstStageMask: PipelineStageFlags,
    pub srcAccessMask: AccessFlags,
    pub dstAccessMask: AccessFlags,
    pub dependencyFlags: DependencyFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassCreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: RenderPassCreateFlags,
    pub attachmentCount: u32,
    pub pAttachments: *const AttachmentDescription,
    pub subpassCount: u32,
    pub pSubpasses: *const SubpassDescription,
    pub dependencyCount: u32,
    pub pDependencies: *const SubpassDependency,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CommandPoolCreateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: CommandPoolCreateFlags,
    pub queueFamilyIndex: u32,
}
#[repr(C)]
pub struct CommandBufferAllocateInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub commandPool: CommandPool,
    pub level: CommandBufferLevel,
    pub commandBufferCount: u32,
}
#[repr(C)]
pub struct CommandBufferInheritanceInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub renderPass: RenderPass,
    pub subpass: u32,
    pub framebuffer: Framebuffer,
    pub occlusionQueryEnable: Bool32,
    pub queryFlags: QueryControlFlags,
    pub pipelineStatistics: QueryPipelineStatisticFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CommandBufferBeginInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: CommandBufferUsageFlags,
    pub pInheritanceInfo: *const CommandBufferInheritanceInfo,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferCopy {
    pub srcOffset: DeviceSize,
    pub dstOffset: DeviceSize,
    pub size: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct ImageSubresourceLayers {
    pub aspectMask: ImageAspectFlags,
    pub mipLevel: u32,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct ImageCopy {
    pub srcSubresource: ImageSubresourceLayers,
    pub srcOffset: Offset3D,
    pub dstSubresource: ImageSubresourceLayers,
    pub dstOffset: Offset3D,
    pub extent: Extent3D,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct ImageBlit {
    pub srcSubresource: ImageSubresourceLayers,
    pub srcOffsets: [Offset3D; 2usize],
    pub dstSubresource: ImageSubresourceLayers,
    pub dstOffsets: [Offset3D; 2usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferImageCopy {
    pub bufferOffset: DeviceSize,
    pub bufferRowLength: u32,
    pub bufferImageHeight: u32,
    pub imageSubresource: ImageSubresourceLayers,
    pub imageOffset: Offset3D,
    pub imageExtent: Extent3D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClearDepthStencilValue {
    pub depth: ::std::os::raw::c_float,
    pub stencil: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClearAttachment {
    pub aspectMask: ImageAspectFlags,
    pub colorAttachment: u32,
    pub clearValue: ClearValue,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClearRect {
    pub rect: Rect2D,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct ImageResolve {
    pub srcSubresource: ImageSubresourceLayers,
    pub srcOffset: Offset3D,
    pub dstSubresource: ImageSubresourceLayers,
    pub dstOffset: Offset3D,
    pub extent: Extent3D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryBarrier {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub srcAccessMask: AccessFlags,
    pub dstAccessMask: AccessFlags,
}
#[repr(C)]
pub struct BufferMemoryBarrier {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub srcAccessMask: AccessFlags,
    pub dstAccessMask: AccessFlags,
    pub srcQueueFamilyIndex: u32,
    pub dstQueueFamilyIndex: u32,
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub size: DeviceSize,
}
#[repr(C)]
pub struct ImageMemoryBarrier {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub srcAccessMask: AccessFlags,
    pub dstAccessMask: AccessFlags,
    pub oldLayout: ImageLayout,
    pub newLayout: ImageLayout,
    pub srcQueueFamilyIndex: u32,
    pub dstQueueFamilyIndex: u32,
    pub image: Image,
    pub subresourceRange: ImageSubresourceRange,
}
#[repr(C)]
pub struct RenderPassBeginInfo {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub renderPass: RenderPass,
    pub framebuffer: Framebuffer,
    pub renderArea: Rect2D,
    pub clearValueCount: u32,
    pub pClearValues: *const ClearValue,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DispatchIndirectCommand {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DrawIndexedIndirectCommand {
    pub indexCount: u32,
    pub instanceCount: u32,
    pub firstIndex: u32,
    pub vertexOffset: i32,
    pub firstInstance: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DrawIndirectCommand {
    pub vertexCount: u32,
    pub instanceCount: u32,
    pub firstVertex: u32,
    pub firstInstance: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayPresentInfoKHR {
    pub sType: StructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub srcRect: Rect2D,
    pub dstRect: Rect2D,
    pub persistent: Bool32,
}

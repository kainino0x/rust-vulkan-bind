use vk::*;
use libc::*;

#[link(name = "vulkan")]
extern "C" {
    pub fn vkCreateInstance(pCreateInfo: *const InstanceCreateInfo,
                            pAllocator: *const AllocationCallbacks,
                            pInstance: *mut Instance) -> Result;
    pub fn vkDestroyInstance(instance: Instance,
                             pAllocator: *const AllocationCallbacks);
    pub fn vkEnumeratePhysicalDevices(instance: Instance,
                                      pPhysicalDeviceCount: *mut uint32_t,
                                      pPhysicalDevices: *mut PhysicalDevice) -> Result;
    pub fn vkGetPhysicalDeviceFeatures(physicalDevice: PhysicalDevice,
                                       pFeatures: *mut PhysicalDeviceFeatures);
    pub fn vkGetPhysicalDeviceFormatProperties(physicalDevice: PhysicalDevice,
                                               format: Format,
                                               pFormatProperties: *mut FormatProperties);
    pub fn vkGetPhysicalDeviceImageFormatProperties(physicalDevice: PhysicalDevice,
                                                    format: Format,
                                                    _type: ImageType,
                                                    tiling: ImageTiling,
                                                    usage: ImageUsageFlags,
                                                    flags: ImageCreateFlags,
                                                    pImageFormatProperties: *mut ImageFormatProperties) -> Result;
    pub fn vkGetPhysicalDeviceProperties(physicalDevice: PhysicalDevice,
                                         pProperties: *mut PhysicalDeviceProperties);
    pub fn vkGetPhysicalDeviceQueueFamilyProperties(physicalDevice: PhysicalDevice,
                                                    pQueueFamilyPropertyCount: *mut uint32_t,
                                                    pQueueFamilyProperties: *mut QueueFamilyProperties);
    pub fn vkGetPhysicalDeviceMemoryProperties(physicalDevice: PhysicalDevice,
                                               pMemoryProperties: *mut PhysicalDeviceMemoryProperties);
    pub fn vkGetInstanceProcAddr(instance: Instance,
                                 pName: *const ::std::os::raw::c_char) -> PFN_vkVoidFunction;
    pub fn vkGetDeviceProcAddr(device: Device,
                               pName: *const ::std::os::raw::c_char) -> PFN_vkVoidFunction;
    pub fn vkCreateDevice(physicalDevice: PhysicalDevice,
                          pCreateInfo: *const DeviceCreateInfo,
                          pAllocator: *const AllocationCallbacks,
                          pDevice: *mut Device) -> Result;
    pub fn vkDestroyDevice(device: Device,
                           pAllocator: *const AllocationCallbacks);
    pub fn vkEnumerateInstanceExtensionProperties(pLayerName: *const ::std::os::raw::c_char,
                                                  pPropertyCount: *mut uint32_t,
                                                  pProperties: *mut ExtensionProperties) -> Result;
    pub fn vkEnumerateDeviceExtensionProperties(physicalDevice: PhysicalDevice,
                                                pLayerName: *const ::std::os::raw::c_char,
                                                pPropertyCount: *mut uint32_t,
                                                pProperties: *mut ExtensionProperties) -> Result;
    pub fn vkEnumerateInstanceLayerProperties(pPropertyCount: *mut uint32_t,
                                              pProperties: *mut LayerProperties) -> Result;
    pub fn vkEnumerateDeviceLayerProperties(physicalDevice: PhysicalDevice,
                                            pPropertyCount: *mut uint32_t,
                                            pProperties: *mut LayerProperties) -> Result;
    pub fn vkGetDeviceQueue(device: Device, queueFamilyIndex: uint32_t,
                            queueIndex: uint32_t, pQueue: *mut Queue);
    pub fn vkQueueSubmit(queue: Queue, submitCount: uint32_t,
                         pSubmits: *const SubmitInfo, fence: Fence) -> Result;
    pub fn vkQueueWaitIdle(queue: Queue) -> Result;
    pub fn vkDeviceWaitIdle(device: Device) -> Result;
    pub fn vkAllocateMemory(device: Device,
                            pAllocateInfo: *const MemoryAllocateInfo,
                            pAllocator: *const AllocationCallbacks,
                            pMemory: *mut DeviceMemory) -> Result;
    pub fn vkFreeMemory(device: Device, memory: DeviceMemory,
                        pAllocator: *const AllocationCallbacks);
    pub fn vkMapMemory(device: Device, memory: DeviceMemory,
                       offset: DeviceSize, size: DeviceSize,
                       flags: MemoryMapFlags,
                       ppData: *mut *mut ::std::os::raw::c_void) -> Result;
    pub fn vkUnmapMemory(device: Device, memory: DeviceMemory);
    pub fn vkFlushMappedMemoryRanges(device: Device,
                                     memoryRangeCount: uint32_t,
                                     pMemoryRanges: *const MappedMemoryRange) -> Result;
    pub fn vkInvalidateMappedMemoryRanges(device: Device,
                                          memoryRangeCount: uint32_t,
                                          pMemoryRanges: *const MappedMemoryRange) -> Result;
    pub fn vkGetDeviceMemoryCommitment(device: Device,
                                       memory: DeviceMemory,
                                       pCommittedMemoryInBytes: *mut DeviceSize);
    pub fn vkBindBufferMemory(device: Device, buffer: Buffer,
                              memory: DeviceMemory,
                              memoryOffset: DeviceSize) -> Result;
    pub fn vkBindImageMemory(device: Device, image: Image,
                             memory: DeviceMemory,
                             memoryOffset: DeviceSize) -> Result;
    pub fn vkGetBufferMemoryRequirements(device: Device, buffer: Buffer,
                                         pMemoryRequirements: *mut MemoryRequirements);
    pub fn vkGetImageMemoryRequirements(device: Device, image: Image,
                                        pMemoryRequirements: *mut MemoryRequirements);
    pub fn vkGetImageSparseMemoryRequirements(device: Device,
                                              image: Image,
                                              pSparseMemoryRequirementCount: *mut uint32_t,
                                              pSparseMemoryRequirements: *mut SparseImageMemoryRequirements);
    pub fn vkGetPhysicalDeviceSparseImageFormatProperties(physicalDevice: PhysicalDevice,
                                                          format: Format,
                                                          _type: ImageType,
                                                          samples: SampleCountFlags,
                                                          usage: ImageUsageFlags,
                                                          tiling: ImageTiling,
                                                          pPropertyCount: *mut uint32_t,
                                                          pProperties: *mut SparseImageFormatProperties);
    pub fn vkQueueBindSparse(queue: Queue, bindInfoCount: uint32_t,
                             pBindInfo: *const BindSparseInfo,
                             fence: Fence) -> Result;
    pub fn vkCreateFence(device: Device,
                         pCreateInfo: *const FenceCreateInfo,
                         pAllocator: *const AllocationCallbacks,
                         pFence: *mut Fence) -> Result;
    pub fn vkDestroyFence(device: Device, fence: Fence,
                          pAllocator: *const AllocationCallbacks);
    pub fn vkResetFences(device: Device, fenceCount: uint32_t,
                         pFences: *const Fence) -> Result;
    pub fn vkGetFenceStatus(device: Device, fence: Fence) -> Result;
    pub fn vkWaitForFences(device: Device, fenceCount: uint32_t,
                           pFences: *const Fence, waitAll: Bool32,
                           timeout: uint64_t) -> Result;
    pub fn vkCreateSemaphore(device: Device,
                             pCreateInfo: *const SemaphoreCreateInfo,
                             pAllocator: *const AllocationCallbacks,
                             pSemaphore: *mut Semaphore) -> Result;
    pub fn vkDestroySemaphore(device: Device, semaphore: Semaphore,
                              pAllocator: *const AllocationCallbacks);
    pub fn vkCreateEvent(device: Device,
                         pCreateInfo: *const EventCreateInfo,
                         pAllocator: *const AllocationCallbacks,
                         pEvent: *mut Event) -> Result;
    pub fn vkDestroyEvent(device: Device, event: Event,
                          pAllocator: *const AllocationCallbacks);
    pub fn vkGetEventStatus(device: Device, event: Event) -> Result;
    pub fn vkSetEvent(device: Device, event: Event) -> Result;
    pub fn vkResetEvent(device: Device, event: Event) -> Result;
    pub fn vkCreateQueryPool(device: Device,
                             pCreateInfo: *const QueryPoolCreateInfo,
                             pAllocator: *const AllocationCallbacks,
                             pQueryPool: *mut QueryPool) -> Result;
    pub fn vkDestroyQueryPool(device: Device, queryPool: QueryPool,
                              pAllocator: *const AllocationCallbacks);
    pub fn vkGetQueryPoolResults(device: Device, queryPool: QueryPool,
                                 firstQuery: uint32_t, queryCount: uint32_t,
                                 dataSize: size_t,
                                 pData: *mut ::std::os::raw::c_void,
                                 stride: DeviceSize,
                                 flags: QueryResultFlags) -> Result;
    pub fn vkCreateBuffer(device: Device,
                          pCreateInfo: *const BufferCreateInfo,
                          pAllocator: *const AllocationCallbacks,
                          pBuffer: *mut Buffer) -> Result;
    pub fn vkDestroyBuffer(device: Device, buffer: Buffer,
                           pAllocator: *const AllocationCallbacks);
    pub fn vkCreateBufferView(device: Device,
                              pCreateInfo: *const BufferViewCreateInfo,
                              pAllocator: *const AllocationCallbacks,
                              pView: *mut BufferView) -> Result;
    pub fn vkDestroyBufferView(device: Device, bufferView: BufferView,
                               pAllocator: *const AllocationCallbacks);
    pub fn vkCreateImage(device: Device,
                         pCreateInfo: *const ImageCreateInfo,
                         pAllocator: *const AllocationCallbacks,
                         pImage: *mut Image) -> Result;
    pub fn vkDestroyImage(device: Device, image: Image,
                          pAllocator: *const AllocationCallbacks);
    pub fn vkGetImageSubresourceLayout(device: Device, image: Image,
                                       pSubresource: *const ImageSubresource,
                                       pLayout: *mut SubresourceLayout);
    pub fn vkCreateImageView(device: Device,
                             pCreateInfo: *const ImageViewCreateInfo,
                             pAllocator: *const AllocationCallbacks,
                             pView: *mut ImageView) -> Result;
    pub fn vkDestroyImageView(device: Device, imageView: ImageView,
                              pAllocator: *const AllocationCallbacks);
    pub fn vkCreateShaderModule(device: Device,
                                pCreateInfo: *const ShaderModuleCreateInfo,
                                pAllocator: *const AllocationCallbacks,
                                pShaderModule: *mut ShaderModule) -> Result;
    pub fn vkDestroyShaderModule(device: Device,
                                 shaderModule: ShaderModule,
                                 pAllocator: *const AllocationCallbacks);
    pub fn vkCreatePipelineCache(device: Device,
                                 pCreateInfo: *const PipelineCacheCreateInfo,
                                 pAllocator: *const AllocationCallbacks,
                                 pPipelineCache: *mut PipelineCache) -> Result;
    pub fn vkDestroyPipelineCache(device: Device,
                                  pipelineCache: PipelineCache,
                                  pAllocator: *const AllocationCallbacks);
    pub fn vkGetPipelineCacheData(device: Device,
                                  pipelineCache: PipelineCache,
                                  pDataSize: *mut size_t,
                                  pData: *mut ::std::os::raw::c_void) -> Result;
    pub fn vkMergePipelineCaches(device: Device, dstCache: PipelineCache,
                                 srcCacheCount: uint32_t,
                                 pSrcCaches: *const PipelineCache) -> Result;
    pub fn vkCreateGraphicsPipelines(device: Device,
                                     pipelineCache: PipelineCache,
                                     createInfoCount: uint32_t,
                                     pCreateInfos: *const GraphicsPipelineCreateInfo,
                                     pAllocator: *const AllocationCallbacks,
                                     pPipelines: *mut Pipeline) -> Result;
    pub fn vkCreateComputePipelines(device: Device,
                                    pipelineCache: PipelineCache,
                                    createInfoCount: uint32_t,
                                    pCreateInfos: *const ComputePipelineCreateInfo,
                                    pAllocator: *const AllocationCallbacks,
                                    pPipelines: *mut Pipeline) -> Result;
    pub fn vkDestroyPipeline(device: Device, pipeline: Pipeline,
                             pAllocator: *const AllocationCallbacks);
    pub fn vkCreatePipelineLayout(device: Device,
                                  pCreateInfo: *const PipelineLayoutCreateInfo,
                                  pAllocator: *const AllocationCallbacks,
                                  pPipelineLayout: *mut PipelineLayout) -> Result;
    pub fn vkDestroyPipelineLayout(device: Device,
                                   pipelineLayout: PipelineLayout,
                                   pAllocator: *const AllocationCallbacks);
    pub fn vkCreateSampler(device: Device,
                           pCreateInfo: *const SamplerCreateInfo,
                           pAllocator: *const AllocationCallbacks,
                           pSampler: *mut Sampler) -> Result;
    pub fn vkDestroySampler(device: Device, sampler: Sampler,
                            pAllocator: *const AllocationCallbacks);
    pub fn vkCreateDescriptorSetLayout(device: Device,
                                       pCreateInfo: *const DescriptorSetLayoutCreateInfo,
                                       pAllocator: *const AllocationCallbacks,
                                       pSetLayout: *mut DescriptorSetLayout) -> Result;
    pub fn vkDestroyDescriptorSetLayout(device: Device,
                                        descriptorSetLayout: DescriptorSetLayout,
                                        pAllocator: *const AllocationCallbacks);
    pub fn vkCreateDescriptorPool(device: Device,
                                  pCreateInfo: *const DescriptorPoolCreateInfo,
                                  pAllocator: *const AllocationCallbacks,
                                  pDescriptorPool: *mut DescriptorPool) -> Result;
    pub fn vkDestroyDescriptorPool(device: Device,
                                   descriptorPool: DescriptorPool,
                                   pAllocator: *const AllocationCallbacks);
    pub fn vkResetDescriptorPool(device: Device,
                                 descriptorPool: DescriptorPool,
                                 flags: DescriptorPoolResetFlags) -> Result;
    pub fn vkAllocateDescriptorSets(device: Device,
                                    pAllocateInfo: *const DescriptorSetAllocateInfo,
                                    pDescriptorSets: *mut DescriptorSet) -> Result;
    pub fn vkFreeDescriptorSets(device: Device,
                                descriptorPool: DescriptorPool,
                                descriptorSetCount: uint32_t,
                                pDescriptorSets: *const DescriptorSet) -> Result;
    pub fn vkUpdateDescriptorSets(device: Device,
                                  descriptorWriteCount: uint32_t,
                                  pDescriptorWrites: *const WriteDescriptorSet,
                                  descriptorCopyCount: uint32_t,
                                  pDescriptorCopies: *const CopyDescriptorSet);
    pub fn vkCreateFramebuffer(device: Device,
                               pCreateInfo: *const FramebufferCreateInfo,
                               pAllocator: *const AllocationCallbacks,
                               pFramebuffer: *mut Framebuffer) -> Result;
    pub fn vkDestroyFramebuffer(device: Device, framebuffer: Framebuffer,
                                pAllocator: *const AllocationCallbacks);
    pub fn vkCreateRenderPass(device: Device,
                              pCreateInfo: *const RenderPassCreateInfo,
                              pAllocator: *const AllocationCallbacks,
                              pRenderPass: *mut RenderPass) -> Result;
    pub fn vkDestroyRenderPass(device: Device, renderPass: RenderPass,
                               pAllocator: *const AllocationCallbacks);
    pub fn vkGetRenderAreaGranularity(device: Device,
                                      renderPass: RenderPass,
                                      pGranularity: *mut Extent2D);
    pub fn vkCreateCommandPool(device: Device,
                               pCreateInfo: *const CommandPoolCreateInfo,
                               pAllocator: *const AllocationCallbacks,
                               pCommandPool: *mut CommandPool) -> Result;
    pub fn vkDestroyCommandPool(device: Device, commandPool: CommandPool,
                                pAllocator: *const AllocationCallbacks);
    pub fn vkResetCommandPool(device: Device, commandPool: CommandPool,
                              flags: CommandPoolResetFlags) -> Result;
    pub fn vkAllocateCommandBuffers(device: Device,
                                    pAllocateInfo: *const CommandBufferAllocateInfo,
                                    pCommandBuffers: *mut CommandBuffer) -> Result;
    pub fn vkFreeCommandBuffers(device: Device, commandPool: CommandPool,
                                commandBufferCount: uint32_t,
                                pCommandBuffers: *const CommandBuffer);
    pub fn vkBeginCommandBuffer(commandBuffer: CommandBuffer,
                                pBeginInfo: *const CommandBufferBeginInfo) -> Result;
    pub fn vkEndCommandBuffer(commandBuffer: CommandBuffer) -> Result;
    pub fn vkResetCommandBuffer(commandBuffer: CommandBuffer,
                                flags: CommandBufferResetFlags) -> Result;
    pub fn vkCmdBindPipeline(commandBuffer: CommandBuffer,
                             pipelineBindPoint: PipelineBindPoint,
                             pipeline: Pipeline);
    pub fn vkCmdSetViewport(commandBuffer: CommandBuffer,
                            firstViewport: uint32_t, viewportCount: uint32_t,
                            pViewports: *const Viewport);
    pub fn vkCmdSetScissor(commandBuffer: CommandBuffer,
                           firstScissor: uint32_t, scissorCount: uint32_t,
                           pScissors: *const Rect2D);
    pub fn vkCmdSetLineWidth(commandBuffer: CommandBuffer,
                             lineWidth: ::std::os::raw::c_float);
    pub fn vkCmdSetDepthBias(commandBuffer: CommandBuffer,
                             depthBiasConstantFactor: ::std::os::raw::c_float,
                             depthBiasClamp: ::std::os::raw::c_float,
                             depthBiasSlopeFactor: ::std::os::raw::c_float);
    pub fn vkCmdSetBlendConstants(commandBuffer: CommandBuffer,
                                  blendConstants: *mut ::std::os::raw::c_float);
    pub fn vkCmdSetDepthBounds(commandBuffer: CommandBuffer,
                               minDepthBounds: ::std::os::raw::c_float,
                               maxDepthBounds: ::std::os::raw::c_float);
    pub fn vkCmdSetStencilCompareMask(commandBuffer: CommandBuffer,
                                      faceMask: StencilFaceFlags,
                                      compareMask: uint32_t);
    pub fn vkCmdSetStencilWriteMask(commandBuffer: CommandBuffer,
                                    faceMask: StencilFaceFlags,
                                    writeMask: uint32_t);
    pub fn vkCmdSetStencilReference(commandBuffer: CommandBuffer,
                                    faceMask: StencilFaceFlags,
                                    reference: uint32_t);
    pub fn vkCmdBindDescriptorSets(commandBuffer: CommandBuffer,
                                   pipelineBindPoint: PipelineBindPoint,
                                   layout: PipelineLayout,
                                   firstSet: uint32_t,
                                   descriptorSetCount: uint32_t,
                                   pDescriptorSets: *const DescriptorSet,
                                   dynamicOffsetCount: uint32_t,
                                   pDynamicOffsets: *const uint32_t);
    pub fn vkCmdBindIndexBuffer(commandBuffer: CommandBuffer,
                                buffer: Buffer, offset: DeviceSize,
                                indexType: IndexType);
    pub fn vkCmdBindVertexBuffers(commandBuffer: CommandBuffer,
                                  firstBinding: uint32_t,
                                  bindingCount: uint32_t,
                                  pBuffers: *const Buffer,
                                  pOffsets: *const DeviceSize);
    pub fn vkCmdDraw(commandBuffer: CommandBuffer, vertexCount: uint32_t,
                     instanceCount: uint32_t, firstVertex: uint32_t,
                     firstInstance: uint32_t);
    pub fn vkCmdDrawIndexed(commandBuffer: CommandBuffer,
                            indexCount: uint32_t, instanceCount: uint32_t,
                            firstIndex: uint32_t, vertexOffset: int32_t,
                            firstInstance: uint32_t);
    pub fn vkCmdDrawIndirect(commandBuffer: CommandBuffer, buffer: Buffer,
                             offset: DeviceSize, drawCount: uint32_t,
                             stride: uint32_t);
    pub fn vkCmdDrawIndexedIndirect(commandBuffer: CommandBuffer,
                                    buffer: Buffer, offset: DeviceSize,
                                    drawCount: uint32_t, stride: uint32_t);
    pub fn vkCmdDispatch(commandBuffer: CommandBuffer, x: uint32_t,
                         y: uint32_t, z: uint32_t);
    pub fn vkCmdDispatchIndirect(commandBuffer: CommandBuffer,
                                 buffer: Buffer, offset: DeviceSize);
    pub fn vkCmdCopyBuffer(commandBuffer: CommandBuffer,
                           srcBuffer: Buffer, dstBuffer: Buffer,
                           regionCount: uint32_t,
                           pRegions: *const BufferCopy);
    pub fn vkCmdCopyImage(commandBuffer: CommandBuffer, srcImage: Image,
                          srcImageLayout: ImageLayout, dstImage: Image,
                          dstImageLayout: ImageLayout,
                          regionCount: uint32_t,
                          pRegions: *const ImageCopy);
    pub fn vkCmdBlitImage(commandBuffer: CommandBuffer, srcImage: Image,
                          srcImageLayout: ImageLayout, dstImage: Image,
                          dstImageLayout: ImageLayout,
                          regionCount: uint32_t, pRegions: *const ImageBlit,
                          filter: Filter);
    pub fn vkCmdCopyBufferToImage(commandBuffer: CommandBuffer,
                                  srcBuffer: Buffer, dstImage: Image,
                                  dstImageLayout: ImageLayout,
                                  regionCount: uint32_t,
                                  pRegions: *const BufferImageCopy);
    pub fn vkCmdCopyImageToBuffer(commandBuffer: CommandBuffer,
                                  srcImage: Image,
                                  srcImageLayout: ImageLayout,
                                  dstBuffer: Buffer, regionCount: uint32_t,
                                  pRegions: *const BufferImageCopy);
    pub fn vkCmdUpdateBuffer(commandBuffer: CommandBuffer,
                             dstBuffer: Buffer, dstOffset: DeviceSize,
                             dataSize: DeviceSize, pData: *const uint32_t);
    pub fn vkCmdFillBuffer(commandBuffer: CommandBuffer,
                           dstBuffer: Buffer, dstOffset: DeviceSize,
                           size: DeviceSize, data: uint32_t);
    pub fn vkCmdClearColorImage(commandBuffer: CommandBuffer,
                                image: Image, imageLayout: ImageLayout,
                                pColor: *const ClearColorValue,
                                rangeCount: uint32_t,
                                pRanges: *const ImageSubresourceRange);
    pub fn vkCmdClearDepthStencilImage(commandBuffer: CommandBuffer,
                                       image: Image,
                                       imageLayout: ImageLayout,
                                       pDepthStencil: *const ClearDepthStencilValue,
                                       rangeCount: uint32_t,
                                       pRanges: *const ImageSubresourceRange);
    pub fn vkCmdClearAttachments(commandBuffer: CommandBuffer,
                                 attachmentCount: uint32_t,
                                 pAttachments: *const ClearAttachment,
                                 rectCount: uint32_t,
                                 pRects: *const ClearRect);
    pub fn vkCmdResolveImage(commandBuffer: CommandBuffer,
                             srcImage: Image, srcImageLayout: ImageLayout,
                             dstImage: Image, dstImageLayout: ImageLayout,
                             regionCount: uint32_t,
                             pRegions: *const ImageResolve);
    pub fn vkCmdSetEvent(commandBuffer: CommandBuffer, event: Event,
                         stageMask: PipelineStageFlags);
    pub fn vkCmdResetEvent(commandBuffer: CommandBuffer, event: Event,
                           stageMask: PipelineStageFlags);
    pub fn vkCmdWaitEvents(commandBuffer: CommandBuffer,
                           eventCount: uint32_t, pEvents: *const Event,
                           srcStageMask: PipelineStageFlags,
                           dstStageMask: PipelineStageFlags,
                           memoryBarrierCount: uint32_t,
                           pMemoryBarriers: *const MemoryBarrier,
                           bufferMemoryBarrierCount: uint32_t,
                           pBufferMemoryBarriers: *const BufferMemoryBarrier,
                           imageMemoryBarrierCount: uint32_t,
                           pImageMemoryBarriers: *const ImageMemoryBarrier);
    pub fn vkCmdPipelineBarrier(commandBuffer: CommandBuffer,
                                srcStageMask: PipelineStageFlags,
                                dstStageMask: PipelineStageFlags,
                                dependencyFlags: DependencyFlags,
                                memoryBarrierCount: uint32_t,
                                pMemoryBarriers: *const MemoryBarrier,
                                bufferMemoryBarrierCount: uint32_t,
                                pBufferMemoryBarriers: *const BufferMemoryBarrier,
                                imageMemoryBarrierCount: uint32_t,
                                pImageMemoryBarriers: *const ImageMemoryBarrier);
    pub fn vkCmdBeginQuery(commandBuffer: CommandBuffer,
                           queryPool: QueryPool, query: uint32_t,
                           flags: QueryControlFlags);
    pub fn vkCmdEndQuery(commandBuffer: CommandBuffer,
                         queryPool: QueryPool, query: uint32_t);
    pub fn vkCmdResetQueryPool(commandBuffer: CommandBuffer,
                               queryPool: QueryPool, firstQuery: uint32_t,
                               queryCount: uint32_t);
    pub fn vkCmdWriteTimestamp(commandBuffer: CommandBuffer,
                               pipelineStage: PipelineStageFlags,
                               queryPool: QueryPool, query: uint32_t);
    pub fn vkCmdCopyQueryPoolResults(commandBuffer: CommandBuffer,
                                     queryPool: QueryPool,
                                     firstQuery: uint32_t,
                                     queryCount: uint32_t,
                                     dstBuffer: Buffer,
                                     dstOffset: DeviceSize,
                                     stride: DeviceSize,
                                     flags: QueryResultFlags);
    pub fn vkCmdPushConstants(commandBuffer: CommandBuffer,
                              layout: PipelineLayout,
                              stageFlags: ShaderStageFlags,
                              offset: uint32_t, size: uint32_t,
                              pValues: *const ::std::os::raw::c_void);
    pub fn vkCmdBeginRenderPass(commandBuffer: CommandBuffer,
                                pRenderPassBegin: *const RenderPassBeginInfo,
                                contents: SubpassContents);
    pub fn vkCmdNextSubpass(commandBuffer: CommandBuffer,
                            contents: SubpassContents);
    pub fn vkCmdEndRenderPass(commandBuffer: CommandBuffer);
    pub fn vkCmdExecuteCommands(commandBuffer: CommandBuffer,
                                commandBufferCount: uint32_t,
                                pCommandBuffers: *const CommandBuffer);
}

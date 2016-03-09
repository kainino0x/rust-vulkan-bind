make_flag!{FormatFeatureFlag; FormatFeatureFlags;
    SAMPLED_IMAGE = 0x00000001,
    STORAGE_IMAGE = 0x00000002,
    STORAGE_IMAGE_ATOMIC = 0x00000004,
    UNIFORM_TEXEL_BUFFER = 0x00000008,
    STORAGE_TEXEL_BUFFER = 0x00000010,
    STORAGE_TEXEL_BUFFER_ATOMIC = 0x00000020,
    VERTEX_BUFFER = 0x00000040,
    COLOR_ATTACHMENT = 0x00000080,
    COLOR_ATTACHMENT_BLEND = 0x00000100,
    DEPTH_STENCIL_ATTACHMENT = 0x00000200,
    BLIT_SRC = 0x00000400,
    BLIT_DST = 0x00000800,
    SAMPLED_IMAGE_FILTER_LINEAR = 0x00001000,
}
make_flag!{ImageUsageFlag; ImageUsageFlags;
    TRANSFER_SRC = 0x00000001,
    TRANSFER_DST = 0x00000002,
    SAMPLED = 0x00000004,
    STORAGE = 0x00000008,
    COLOR_ATTACHMENT = 0x00000010,
    DEPTH_STENCIL_ATTACHMENT = 0x00000020,
    TRANSIENT_ATTACHMENT = 0x00000040,
    INPUT_ATTACHMENT = 0x00000080,
}
make_flag!{ImageCreateFlag; ImageCreateFlags;
    SPARSE_BINDING = 0x00000001,
    SPARSE_RESIDENCY = 0x00000002,
    SPARSE_ALIASED = 0x00000004,
    MUTABLE_FORMAT = 0x00000008,
    CUBE_COMPATIBLE = 0x00000010,
}
make_flag!{SampleCountFlag; SampleCountFlags;
    E_1 = 0x00000001,
    E_2 = 0x00000002,
    E_4 = 0x00000004,
    E_8 = 0x00000008,
    E_16 = 0x00000010,
    E_32 = 0x00000020,
    E_64 = 0x00000040,
}
make_flag!{QueueFlag; QueueFlags;
    GRAPHICS = 0x00000001,
    COMPUTE = 0x00000002,
    TRANSFER = 0x00000004,
    SPARSE_BINDING = 0x00000008,
}
make_flag!{MemoryPropertyFlag; MemoryPropertyFlags;
    DEVICE_LOCAL = 0x00000001,
    HOST_VISIBLE = 0x00000002,
    HOST_COHERENT = 0x00000004,
    HOST_CACHED = 0x00000008,
    LAZILY_ALLOCATED = 0x00000010,
}
make_flag!{MemoryHeapFlag; MemoryHeapFlags;
    DEVICE_LOCAL = 0x00000001,
}
make_flag!{PipelineStageFlag; PipelineStageFlags;
    TOP_OF_PIPE = 0x00000001,
    DRAW_INDIRECT = 0x00000002,
    VERTEX_INPUT = 0x00000004,
    VERTEX_SHADER = 0x00000008,
    TESSELLATION_CONTROL_SHADER = 0x00000010,
    TESSELLATION_EVALUATION_SHADER = 0x00000020,
    GEOMETRY_SHADER = 0x00000040,
    FRAGMENT_SHADER = 0x00000080,
    EARLY_FRAGMENT_TESTS = 0x00000100,
    LATE_FRAGMENT_TESTS = 0x00000200,
    COLOR_ATTACHMENT_OUTPUT = 0x00000400,
    COMPUTE_SHADER = 0x00000800,
    TRANSFER = 0x00001000,
    BOTTOM_OF_PIPE = 0x00002000,
    HOST = 0x00004000,
    ALL_GRAPHICS = 0x00008000,
    ALL_COMMANDS = 0x00010000,
}
make_flag!{ImageAspectFlag; ImageAspectFlags;
    COLOR = 0x00000001,
    DEPTH = 0x00000002,
    STENCIL = 0x00000004,
    METADATA = 0x00000008,
}
make_flag!{SparseImageFormatFlag; SparseImageFormatFlags;
    SINGLE_MIPTAIL = 0x00000001,
    ALIGNED_MIP_SIZE = 0x00000002,
    NONSTANDARD_BLOCK_SIZE = 0x00000004,
}
make_flag!{SparseMemoryBindFlag; SparseMemoryBindFlags;
    METADATA = 0x00000001,
}
make_flag!{FenceCreateFlag; FenceCreateFlags;
    SIGNALED = 0x00000001,
}
make_flag!{QueryPipelineStatisticFlag; QueryPipelineStatisticFlags;
    INPUT_ASSEMBLY_VERTICES = 0x00000001,
    INPUT_ASSEMBLY_PRIMITIVES = 0x00000002,
    VERTEX_SHADER_INVOCATIONS = 0x00000004,
    GEOMETRY_SHADER_INVOCATIONS = 0x00000008,
    GEOMETRY_SHADER_PRIMITIVES = 0x00000010,
    CLIPPING_INVOCATIONS = 0x00000020,
    CLIPPING_PRIMITIVES = 0x00000040,
    FRAGMENT_SHADER_INVOCATIONS = 0x00000080,
    TESSELLATION_CONTROL_SHADER_PATCHES = 0x00000100,
    TESSELLATION_EVALUATION_SHADER_INVOCATIONS = 0x00000200,
    COMPUTE_SHADER_INVOCATIONS = 0x00000400,
}
make_flag!{QueryResultFlag; QueryResultFlags;
    E_64 = 0x00000001,
    WAIT = 0x00000002,
    WITH_AVAILABILITY = 0x00000004,
    PARTIAL = 0x00000008,
}
make_flag!{BufferCreateFlag; BufferCreateFlags;
    SPARSE_BINDING = 0x00000001,
    SPARSE_RESIDENCY = 0x00000002,
    SPARSE_ALIASED = 0x00000004,
}
make_flag!{BufferUsageFlag; BufferUsageFlags;
    TRANSFER_SRC = 0x00000001,
    TRANSFER_DST = 0x00000002,
    UNIFORM_TEXEL_BUFFER = 0x00000004,
    STORAGE_TEXEL_BUFFER = 0x00000008,
    UNIFORM_BUFFER = 0x00000010,
    STORAGE_BUFFER = 0x00000020,
    INDEX_BUFFER = 0x00000040,
    VERTEX_BUFFER = 0x00000080,
    INDIRECT_BUFFER = 0x00000100,
}
make_flag!{PipelineCreateFlag; PipelineCreateFlags;
    DISABLE_OPTIMIZATION = 0x00000001,
    ALLOW_DERIVATIVES = 0x00000002,
    DERIVATIVE = 0x00000004,
}
make_flag!{ShaderStageFlag; ShaderStageFlags;
    VERTEX = 0x00000001,
    TESSELLATION_CONTROL = 0x00000002,
    TESSELLATION_EVALUATION = 0x00000004,
    GEOMETRY = 0x00000008,
    FRAGMENT = 0x00000010,
    COMPUTE = 0x00000020,
    ALL_GRAPHICS = 0x1F,
    ALL = 0x7FFFFFFF,
}
make_flag!{CullModeFlag; CullModeFlags;
    NONE = 0,
    FRONT = 0x00000001,
    BACK = 0x00000002,
    FRONT_AND_BACK = 0x3,
}
make_flag!{ColorComponentFlag; ColorComponentFlags;
    R = 0x00000001,
    G = 0x00000002,
    B = 0x00000004,
    A = 0x00000008,
}
make_flag!{DescriptorPoolCreateFlag; DescriptorPoolCreateFlags;
    FREE_DESCRIPTOR_SET = 0x00000001,
}
make_flag!{AttachmentDescriptionFlag; AttachmentDescriptionFlags;
    MAY_ALIAS = 0x00000001,
}
make_flag!{AccessFlag; AccessFlags;
    INDIRECT_COMMAND_READ = 0x00000001,
    INDEX_READ = 0x00000002,
    VERTEX_ATTRIBUTE_READ = 0x00000004,
    UNIFORM_READ = 0x00000008,
    INPUT_ATTACHMENT_READ = 0x00000010,
    SHADER_READ = 0x00000020,
    SHADER_WRITE = 0x00000040,
    COLOR_ATTACHMENT_READ = 0x00000080,
    COLOR_ATTACHMENT_WRITE = 0x00000100,
    DEPTH_STENCIL_ATTACHMENT_READ = 0x00000200,
    DEPTH_STENCIL_ATTACHMENT_WRITE = 0x00000400,
    TRANSFER_READ = 0x00000800,
    TRANSFER_WRITE = 0x00001000,
    HOST_READ = 0x00002000,
    HOST_WRITE = 0x00004000,
    MEMORY_READ = 0x00008000,
    MEMORY_WRITE = 0x00010000,
}
make_flag!{DependencyFlag; DependencyFlags;
    BY_REGION = 0x00000001,
}
make_flag!{CommandPoolCreateFlag; CommandPoolCreateFlags;
    TRANSIENT = 0x00000001,
    RESET_COMMAND_BUFFER = 0x00000002,
}
make_flag!{CommandPoolResetFlag; CommandPoolResetFlags;
    RELEASE_RESOURCES = 0x00000001,
}
make_flag!{CommandBufferUsageFlag; CommandBufferUsageFlags;
    ONE_TIME_SUBMIT = 0x00000001,
    RENDER_PASS_CONTINUE = 0x00000002,
    SIMULTANEOUS_USE = 0x00000004,
}
make_flag!{QueryControlFlag; QueryControlFlags;
    PRECISE = 0x00000001,
}
make_flag!{CommandBufferResetFlag; CommandBufferResetFlags;
    RELEASE_RESOURCES = 0x00000001,
}
make_flag!{StencilFaceFlag; StencilFaceFlags;
    FACE_FRONT = 0x00000001,
    FACE_BACK = 0x00000002,
    FRONT_AND_BACK = 0x3,
}
make_flag!{InstanceCreateFlag; InstanceCreateFlags; }
make_flag!{DeviceCreateFlag; DeviceCreateFlags; }
make_flag!{DeviceQueueCreateFlag; DeviceQueueCreateFlags; }
make_flag!{MemoryMapFlag; MemoryMapFlags; }
make_flag!{SemaphoreCreateFlag; SemaphoreCreateFlags; }
make_flag!{EventCreateFlag; EventCreateFlags; }
make_flag!{QueryPoolCreateFlag; QueryPoolCreateFlags; }
make_flag!{BufferViewCreateFlag; BufferViewCreateFlags; }
make_flag!{ImageViewCreateFlag; ImageViewCreateFlags; }
make_flag!{ShaderModuleCreateFlag; ShaderModuleCreateFlags; }
make_flag!{PipelineCacheCreateFlag; PipelineCacheCreateFlags; }
make_flag!{PipelineShaderStageCreateFlag; PipelineShaderStageCreateFlags; }
make_flag!{PipelineVertexInputStateCreateFlag; PipelineVertexInputStateCreateFlags; }
make_flag!{PipelineInputAssemblyStateCreateFlag; PipelineInputAssemblyStateCreateFlags; }
make_flag!{PipelineTessellationStateCreateFlag; PipelineTessellationStateCreateFlags; }
make_flag!{PipelineViewportStateCreateFlag; PipelineViewportStateCreateFlags; }
make_flag!{PipelineRasterizationStateCreateFlag; PipelineRasterizationStateCreateFlags; }
make_flag!{PipelineMultisampleStateCreateFlag; PipelineMultisampleStateCreateFlags; }
make_flag!{PipelineDepthStencilStateCreateFlag; PipelineDepthStencilStateCreateFlags; }
make_flag!{PipelineColorBlendStateCreateFlag; PipelineColorBlendStateCreateFlags; }
make_flag!{PipelineDynamicStateCreateFlag; PipelineDynamicStateCreateFlags; }
make_flag!{PipelineLayoutCreateFlag; PipelineLayoutCreateFlags; }
make_flag!{SamplerCreateFlag; SamplerCreateFlags; }
make_flag!{DescriptorSetLayoutCreateFlag; DescriptorSetLayoutCreateFlags; }
make_flag!{DescriptorPoolResetFlag; DescriptorPoolResetFlags; }
make_flag!{FramebufferCreateFlag; FramebufferCreateFlags; }
make_flag!{RenderPassCreateFlag; RenderPassCreateFlags; }
make_flag!{SubpassDescriptionFlag; SubpassDescriptionFlags; }

pub use ::vulkanalia::loader::LIBRARY as VULKAN_LIBRARY_FILE_NAME;
pub use ::vulkanalia::loader::LibloadingLoader as VulkanLibraryLoader;
pub use ::vulkanalia::window as VulkanWindow;
pub use ::vulkanalia::vk;
pub use ::vulkanalia::vk::Bool32 as VulkanBool32;
pub use ::vulkanalia::vk::Handle as VulkanHandler;
pub use ::vulkanalia::vk::HasBuilder as VulkanBuilderHas;
pub use ::vulkanalia::vk::ApplicationInfo as VulkanApplicationInfomation;
pub use ::vulkanalia::vk::ApplicationInfoBuilder as VulkanApplicationInformationBuilder;
pub use ::vulkanalia::vk::InstanceCreateInfo as VulkanInstanceCreateInformation;
pub use ::vulkanalia::vk::ExtensionName as VulkanExtensionName;
pub use ::vulkanalia::vk::ExtDebugUtilsExtension as VulkanExtensionDebugUtility;
pub use ::vulkanalia::vk::EXT_DEBUG_UTILS_EXTENSION as VULKAN_EXTENSION_DEBUG_UTILITY;
pub use ::vulkanalia::vk::DebugUtilsMessengerEXT as VulkanExtensionDebugUtilityMessenger;
pub use ::vulkanalia::vk::DebugUtilsMessengerCreateInfoEXT as VulkanExtensionDebugUtilityMessengerCreateInformation;
pub use ::vulkanalia::vk::DebugUtilsMessengerCreateInfoEXTBuilder as VulkanExtensionDebugUtilityMessengerCreateInformationBuilder;
pub use ::vulkanalia::vk::DebugUtilsMessageSeverityFlagsEXT as VulkanExtensionDebugUtilityMessageSeverityFlagS;
pub use ::vulkanalia::vk::DebugUtilsMessageTypeFlagsEXT as VulkanExtensionDebugUtilityMessageTypeFlagS;
pub use ::vulkanalia::vk::DebugUtilsMessengerCallbackDataEXT as VulkanExtensionDebugUtilityMessengerCallbackData;
pub use ::vulkanalia::vk::PhysicalDevice as VulkanDevicePhysical;
pub use ::vulkanalia::vk::Queue as VulkanQueue;
pub use ::vulkanalia::vk::QueueFlags as VulkanQueueFlagS;
pub use ::vulkanalia::vk::DeviceCreateInfo as VulkanDeviceLogicalCreateInformation;
pub use ::vulkanalia::vk::DeviceCreateInfoBuilder as VulkanDeviceLogicalCreateInformationBuilder;
pub use ::vulkanalia::vk::DeviceQueueCreateInfo as VulkanDeviceLogicalQueueCreateInformation;
pub use ::vulkanalia::vk::DeviceQueueCreateInfoBuilder as VulkanDeviceLogicalQueueCreateInformationBuilder;
pub use ::vulkanalia::vk::PhysicalDeviceFeatures as VulkanDevicePhysicalFeatureS;
pub use ::vulkanalia::vk::PhysicalDeviceFeaturesBuilder as VulkanDevicePhysicalFeatureSBuilder;

pub use ::vulkanalia::vk::KhrSurfaceExtension as VulkanSurfaceExtensionKhr;
pub use ::vulkanalia::vk::SurfaceKHR as VulkanSurfaceKhr;
pub use ::vulkanalia::vk::Win32SurfaceCreateInfoKHR as VulkanSurfaceCreateInformationKhr;
pub use ::vulkanalia::vk::Win32SurfaceCreateInfoKHRBuilder as VulkanSurfaceCreateInformationBuilderKhr;
pub use ::vulkanalia::vk::KhrWin32SurfaceExtension as VulkanSurfaceExtensionWin32;
pub use ::vulkanalia::vk::SurfaceCapabilitiesKHR as VulkanSurfaceCapabilitySKhr;
pub use ::vulkanalia::vk::SurfaceFormatKHR as VulkanSurfaceFormatKhr;
pub use ::vulkanalia::vk::PresentModeKHR as VulkanPresentModeKhr;

pub use ::vulkanalia::vk::KhrSwapchainExtension as VulkanSwapchainExtensionKhr;
pub use ::vulkanalia::vk::SwapchainKHR as VulkanSwapchainKhr;
pub use ::vulkanalia::vk::Format as VulkanFormat;
pub use ::vulkanalia::vk::ColorSpaceKHR as VulkanColorSpaceKhr;
pub use ::vulkanalia::vk::Extent2D as VulkanExtentD2;
pub use ::vulkanalia::vk::Image as VulkanImage;
pub use ::vulkanalia::vk::SharingMode as VulkanSharingMode;
pub use ::vulkanalia::vk::SwapchainCreateInfoKHR as VulkanSwapchainCreateInformationKhr;
pub use ::vulkanalia::vk::SwapchainCreateInfoKHRBuilder as VulkanSwapchainCreateInformationBuilderKhr;
pub use ::vulkanalia::vk::ImageUsageFlags as VulkanImageUsageFlagS;
pub use ::vulkanalia::vk::CompositeAlphaFlagsKHR as VulkanCompositeAlphaFlagSKhr;

pub use ::vulkanalia::vk::ATTACHMENT_UNUSED as VULKAN_ATTACHMENT_UNUSED;
pub use ::vulkanalia::vk::FALSE as VULKAN_FALSE;
pub use ::vulkanalia::vk::LOD_CLAMP_NONE as VULKAN_LOD_CLAMP_NONE;
pub use ::vulkanalia::vk::LUID_SIZE as VULKAN_LUID_SIZE;
pub use ::vulkanalia::vk::MAX_DESCRIPTION_SIZE as VULKAN_MAX_DESCRIPTION_SIZE;
pub use ::vulkanalia::vk::MAX_DEVICE_GROUP_SIZE as VULKAN_MAX_DEVICE_GROUP_SIZE;
pub use ::vulkanalia::vk::MAX_DRIVER_INFO_SIZE as VULKAN_MAX_DRIVER_INFO_SIZE;
pub use ::vulkanalia::vk::MAX_DRIVER_NAME_SIZE as VULKAN_MAX_DRIVER_NAME_SIZE;
pub use ::vulkanalia::vk::MAX_EXTENSION_NAME_SIZE as VULKAN_MAX_EXTENSION_NAME_SIZE;
pub use ::vulkanalia::vk::MAX_GLOBAL_PRIORITY_SIZE_EXT as VULKAN_MAX_GLOBAL_PRIORITY_SIZE_EXT;
pub use ::vulkanalia::vk::MAX_MEMORY_HEAPS as VULKAN_MAX_MEMORY_HEAPS;
pub use ::vulkanalia::vk::MAX_MEMORY_TYPES as VULKAN_MAX_MEMORY_TYPES;
pub use ::vulkanalia::vk::MAX_PHYSICAL_DEVICE_NAME_SIZE as VULKAN_MAX_PHYSICAL_DEVICE_NAME_SIZE;
pub use ::vulkanalia::vk::QUEUE_FAMILY_EXTERNAL as VULKAN_QUEUE_FAMILY_EXTERNAL;
pub use ::vulkanalia::vk::QUEUE_FAMILY_FOREIGN_EXT as VULKAN_QUEUE_FAMILY_FOREIGN_EXT;
pub use ::vulkanalia::vk::QUEUE_FAMILY_IGNORED as VULKAN_QUEUE_FAMILY_IGNORED;
pub use ::vulkanalia::vk::REMAINING_ARRAY_LAYERS as VULKAN_REMAINING_ARRAY_LAYERS;
pub use ::vulkanalia::vk::REMAINING_MIP_LEVELS as VULKAN_REMAINING_MIP_LEVELS;
pub use ::vulkanalia::vk::SHADER_UNUSED_KHR as VULKAN_SHADER_UNUSED_KHR;
pub use ::vulkanalia::vk::SUBPASS_EXTERNAL as VULKAN_SUBPASS_EXTERNAL;
pub use ::vulkanalia::vk::TRUE as VULKAN_TRUE;
pub use ::vulkanalia::vk::UUID_SIZE as VULKAN_UUID_SIZE;
pub use ::vulkanalia::vk::WHOLE_SIZE as VULKAN_WHOLE_SIZE;
pub use ::vulkanalia::vk::KHR_SWAPCHAIN_EXTENSION as VULKAN_SWAPCHAIN_EXTENSION_KHR;

pub use ::vulkanalia::Device as VulkanDevice;
pub use ::vulkanalia::Entry as VulkanEntry;
pub use ::vulkanalia::Instance as VulkanInstance;
pub use ::vulkanalia::VkResult as VulkanResult;
pub use ::vulkanalia::VkSuccessResult as VulkanResultSuccess;
pub use ::vulkanalia::vk::DeviceV1_0 as VulkanDeviceVersion1_0;
pub use ::vulkanalia::vk::EntryV1_0 as VulkanEntryVersion1_0;
pub use ::vulkanalia::vk::InstanceV1_0 as VulkanInstanceVersion1_0;
pub use ::vulkanalia::vk::DeviceV1_1 as VulkanDeviceVersion1_1;
pub use ::vulkanalia::vk::EntryV1_1 as VulkanEntryVersion1_1;
pub use ::vulkanalia::vk::InstanceV1_1 as VulkanInstanceVersion1_1;
pub use ::vulkanalia::vk::DeviceV1_2 as VulkanDeviceVersion1_2;
pub use ::vulkanalia::vk::EntryV1_2 as VulkanEntryVersion1_2;
pub use ::vulkanalia::vk::InstanceV1_2 as VulkanInstanceVersion1_2;

pub mod prelude {
    pub mod version1_0 {
        pub use ::vulkanalia::vk::Handle as VulkanHandler;
        pub use ::vulkanalia::vk::HasBuilder as VulkanBuilderHas;
        pub use ::vulkanalia::Device as VulkanDeviceLogical;
        pub use ::vulkanalia::Entry as VulkanEntry;
        pub use ::vulkanalia::Instance as VulkanInstance;
        pub use ::vulkanalia::VkResult as VulkanResult;
        pub use ::vulkanalia::VkSuccessResult as VulkanResultSuccess;
        pub use ::vulkanalia::vk::DeviceV1_0 as VulkanDeviceVersion1_0;
        pub use ::vulkanalia::vk::EntryV1_0 as VulkanEntryVersion1_0;
        pub use ::vulkanalia::vk::InstanceV1_0 as VulkanInstanceVersion1_0;
    }

    pub mod version1_1 {
        pub use crate::prelude::version1_0::*;
        pub use ::vulkanalia::vk::DeviceV1_1 as VulkanDeviceVersion1_1;
        pub use ::vulkanalia::vk::EntryV1_1 as VulkanEntryVersion1_1;
        pub use ::vulkanalia::vk::InstanceV1_1 as VulkanInstanceVersion1_1;
    }

    pub mod version1_2 {
        pub use crate::prelude::version1_0::*;
        pub use ::vulkanalia::vk::DeviceV1_2 as VulkanDeviceVersion1_2;
        pub use ::vulkanalia::vk::EntryV1_2 as VulkanEntryVersion1_2;
        pub use ::vulkanalia::vk::InstanceV1_2 as VulkanInstanceVersion1_2;
    }
}


#[derive(Clone, Copy)]
pub struct VulkanErrorCode(i32);

impl VulkanErrorCode {
    pub fn new(code: i32) -> Self {
        Self(code)
    }
}

#[derive(Clone, Copy)]
pub struct VulkanQueueFamilyIndexGraphic(u32);

impl VulkanQueueFamilyIndexGraphic {
    pub fn new(queue_index: u32) -> Self {
        Self(queue_index)
    }

    pub fn as_raw(&self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy)]
pub struct VulkanQueueFamilyIndexSurface(u32);

impl VulkanQueueFamilyIndexSurface {
    pub fn new(queue_index: u32) -> Self {
        Self(queue_index)
    }

    pub fn as_raw(&self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy)]
pub struct VulkanSwapchainImageCount(u32);

impl VulkanSwapchainImageCount {
    pub fn new(image_count: u32) -> Self {
        Self(image_count)
    }

    pub fn as_raw(&self) -> u32 {
        self.0
    }
}
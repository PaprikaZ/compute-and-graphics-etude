use ::libloading::Error as LibraryLoadingError;
use ::window_uniform::WindowUniformErrorOperationSystem;
use ::vulkan::VulkanErrorCode;


pub enum TerminationProcessMain {
    Ok,
    InitializationLoggerConsoleFail,
    InitializationWindowUniformFail(WindowUniformErrorOperationSystem),
    InitializationVulkanLibraryLoadingFail(LibraryLoadingError),
    InitializationVulkanEntryCreateFail(Box<dyn std::error::Error + Send + Sync + 'static>),
    InitializationVulkanInstanceCreateFail(VulkanErrorCode),
    InitializationVulkanValidationLayerNotSupport,
    InitializationVulkanEnumeratePhysicalDeviceFail(VulkanErrorCode),
    InitializationVulkanDevicePhysicalAllNotQualified,
    InitializationVulkanDeviceLogicalCreateFail(VulkanErrorCode),
    InitializationVulkanSurfaceCreateFail(VulkanErrorCode),
    InitializationVulkanDevicePhysicalSurfaceCapabilitySGetFail(VulkanErrorCode),
    InitializationVulkanDevicePhysicalSurfaceFormatSGetFail(VulkanErrorCode),
    InitializationVulkanDevicePhysicalSurfacePresentModeSGetFail(VulkanErrorCode),
    InitializationVulkanSwapchainCreateFail(VulkanErrorCode),
    InitializationVulkanSwapchainImageSGetFail(VulkanErrorCode),
    InitializationVulkanDevicePhysicalExtensionPropertySEnumerateFail(VulkanErrorCode),
}

impl TerminationProcessMain {
    fn to_exit_code(self) -> u8 {
        match self {
            Self::Ok => 0u8,
            Self::InitializationLoggerConsoleFail => 1u8,
            Self::InitializationWindowUniformFail(_) => 2u8,
            Self::InitializationVulkanLibraryLoadingFail(_) => 3u8,
            Self::InitializationVulkanEntryCreateFail(_) => 4u8,
            Self::InitializationVulkanInstanceCreateFail(_) => 5u8,
            Self::InitializationVulkanValidationLayerNotSupport => 6u8,
            Self::InitializationVulkanEnumeratePhysicalDeviceFail(_) => 7u8,
            Self::InitializationVulkanDevicePhysicalAllNotQualified => 8u8,
            Self::InitializationVulkanDeviceLogicalCreateFail(_) => 9u8,
            Self::InitializationVulkanSurfaceCreateFail(_) => 10u8,
            Self::InitializationVulkanDevicePhysicalSurfaceCapabilitySGetFail(_) => 11u8,
            Self::InitializationVulkanDevicePhysicalSurfaceFormatSGetFail(_) => 12u8,
            Self::InitializationVulkanDevicePhysicalSurfacePresentModeSGetFail(_) => 13u8,
            Self::InitializationVulkanSwapchainCreateFail(_) => 14u8,
            Self::InitializationVulkanSwapchainImageSGetFail(_) => 15u8,
            Self::InitializationVulkanDevicePhysicalExtensionPropertySEnumerateFail(_) => 16u8,
        }
    }
}

impl std::process::Termination for TerminationProcessMain {
    fn report(self) -> std::process::ExitCode {
        std::process::ExitCode::from(self.to_exit_code())
    }
}
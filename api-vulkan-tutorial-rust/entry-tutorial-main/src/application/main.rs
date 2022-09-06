use ::window_uniform::prelude::*;
use ::vulkan::VulkanExtensionName;
use ::vulkan::VulkanExtensionDebugUtilityMessenger;
use ::vulkan::prelude::version1_2::*;
use ::vulkan::VulkanErrorCode;
use ::vulkan::VulkanQueue;
use ::vulkan::VulkanDevicePhysical;
use ::vulkan::VulkanSurfaceKhr;
use ::vulkan::VulkanFormat;
use ::vulkan::VulkanExtentD2;
use ::vulkan::VulkanSwapchainKhr;
use ::vulkan::VulkanImage;
use ::vulkan::VulkanImageView;
use ::vulkan::VulkanRenderPass;
use ::vulkan::VulkanPipelineLayout;
use ::vulkan::VulkanPipeline;
use ::vulkan::VulkanFrameBuffer;
use ::vulkan::VulkanCommandPool;
use ::vulkan::VulkanCommandBuffer;
use ::vulkan::VulkanSemaphore;
use ::vulkan::VulkanFence;
use ::vulkan::VulkanExtensionDebugUtility;
use ::vulkan::VulkanSurfaceExtensionKhr;
use ::vulkan::VulkanSwapchainExtensionKhr;
use ::vulkan::VulkanPipelineStageFlagS;
use ::vulkan::VulkanSubmitInformation;
use ::vulkan::VulkanPresentInformationKhr;

use crate::config::VULKAN_FRAME_IN_FLIGHT_MAX;
use crate::termination::TerminationProcessMain;
use crate::application::vulkan_instance_validation_wi::ApplicationVulkanInstanceValidationWi;
use crate::application::vulkan_instance_validation_wo::ApplicationVulkanInstanceValidationWo;


pub struct Application {
    pub vulkan_entry: VulkanEntry,
    pub vulkan_instance: VulkanInstance,
    pub vulkan_debug_messenger: Option<VulkanExtensionDebugUtilityMessenger>,
    pub vulkan_device_physical: VulkanDevicePhysical,
    pub vulkan_device_logical: VulkanDeviceLogical,
    pub vulkan_queue_graphic: VulkanQueue,
    pub vulkan_surface: VulkanSurfaceKhr,
    pub vulkan_queue_present: VulkanQueue,
    pub vulkan_swapchain_format: VulkanFormat,
    pub vulkan_swapchain_extent: VulkanExtentD2,
    pub vulkan_swapchain: VulkanSwapchainKhr,
    pub vulkan_swapchain_image_s: Vec<VulkanImage>,
    pub vulkan_swapchain_image_view_s: Vec<VulkanImageView>,
    pub vulkan_render_pass: VulkanRenderPass,
    pub vulkan_pipeline_layout: VulkanPipelineLayout,
    pub vulkan_pipeline: VulkanPipeline,
    pub vulkan_frame_buffer_s: Vec<VulkanFrameBuffer>,
    pub vulkan_command_pool: VulkanCommandPool,
    pub vulkan_command_buffer_s: Vec<VulkanCommandBuffer>,
    pub vulkan_semaphore_s_image_available: Vec<VulkanSemaphore>,
    pub vulkan_semaphore_s_render_finished: Vec<VulkanSemaphore>,
    pub vulkan_fence_s_in_flight_slide: Vec<VulkanFence>,
    pub vulkan_fence_s_in_flight_image: Vec<VulkanFence>,
    pub vulkan_frame_index_current: usize,
}

impl Application {
    pub unsafe fn create(
        window: &WindowUniformWindow,
        optional_validation_layer: Option<&VulkanExtensionName>,
        vulkan_physical_device_extension_s: &[VulkanExtensionName])
     -> Result<Self, TerminationProcessMain>
    {
        match optional_validation_layer {
            None =>
                ApplicationVulkanInstanceValidationWo::create(window, vulkan_physical_device_extension_s),
            Some(validation_layer) =>
                ApplicationVulkanInstanceValidationWi::create(window, validation_layer, vulkan_physical_device_extension_s),
        }
    }

    pub unsafe fn render(&mut self, _window: &WindowUniformWindow) -> Result<(), TerminationProcessMain> {
        let vulkan_slide_in_flight_fence = self.vulkan_fence_s_in_flight_slide[self.vulkan_frame_index_current];
        let vulkan_image_in_flight_fence = self.vulkan_fence_s_in_flight_image[self.vulkan_frame_index_current];
        let vulkan_available_image_semaphore = self.vulkan_semaphore_s_image_available[self.vulkan_frame_index_current];
        let wait_vulkan_in_flight_fence_result =
            self.vulkan_device_logical.wait_for_fences(&[vulkan_slide_in_flight_fence], true, u64::max_value());
        let _ =
            match wait_vulkan_in_flight_fence_result {
                Err(error) => {
                    let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                    return Err(TerminationProcessMain::InitializationVulkanFenceWaitFail(vulkan_error_code));
                },
                Ok(success_code) => success_code,
            };
        let acquire_vulkan_next_image_index_result =
            self.vulkan_device_logical.acquire_next_image_khr(
                self.vulkan_swapchain, u64::max_value(), vulkan_available_image_semaphore, VulkanFence::null());
        let vulkan_next_image_index =
            match acquire_vulkan_next_image_index_result {
                Err(error) => {
                    let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                    return Err(TerminationProcessMain::InitializationVulkanAcquireNextImageFail(vulkan_error_code));
                },
                Ok((image_index, _success_code)) => image_index as usize,
            };
        if !vulkan_image_in_flight_fence.is_null() {
            let wait_vulkan_unknown_fence_result =
                self.vulkan_device_logical.wait_for_fences(&[vulkan_image_in_flight_fence], true, u64::max_value());
            match wait_vulkan_unknown_fence_result {
                Err(error) => {
                    let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                    return Err(TerminationProcessMain::InitializationVulkanFenceWaitFail(vulkan_error_code));
                },
                Ok(_success_code) => (),
            }
        }
        self.vulkan_fence_s_in_flight_image[vulkan_next_image_index] = vulkan_slide_in_flight_fence;
        let wait_vulkan_semaphore_s = &[self.vulkan_semaphore_s_image_available[self.vulkan_frame_index_current]];
        let wait_vulkan_pipeline_stage_flag_s = &[VulkanPipelineStageFlagS::COLOR_ATTACHMENT_OUTPUT];
        let vulkan_command_buffer_s = &[self.vulkan_command_buffer_s[vulkan_next_image_index]];
        let vulkan_signal_semaphore_s = &[self.vulkan_semaphore_s_render_finished[self.vulkan_frame_index_current]];
        let vulkan_submit_information =
            VulkanSubmitInformation::builder()
            .wait_semaphores(wait_vulkan_semaphore_s)
            .wait_dst_stage_mask(wait_vulkan_pipeline_stage_flag_s)
            .command_buffers(vulkan_command_buffer_s)
            .signal_semaphores(vulkan_signal_semaphore_s);
        let reset_vulkan_fence_s_result =
            self.vulkan_device_logical.reset_fences(&[vulkan_slide_in_flight_fence]);
        match reset_vulkan_fence_s_result {
            Err(error) => {
                let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                return Err(TerminationProcessMain::InitializationVulkanFenceResetFail(vulkan_error_code));
            },
            Ok(()) => (),
        };
        let submit_vulkan_queue_result =
            self.vulkan_device_logical.queue_submit(self.vulkan_queue_graphic, &[vulkan_submit_information], vulkan_slide_in_flight_fence);
        match submit_vulkan_queue_result {
            Err(error) => {
                let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                return Err(TerminationProcessMain::InitializationVulkanQueueSubmitFail(vulkan_error_code));
            },
            Ok(()) => (),
        };
        let vulkan_swapchain_s = &[self.vulkan_swapchain];
        let vulkan_image_index_s = &[vulkan_next_image_index as u32];
        let vulkan_present_information =
            VulkanPresentInformationKhr::builder()
            .wait_semaphores(vulkan_signal_semaphore_s)
            .swapchains(vulkan_swapchain_s)
            .image_indices(vulkan_image_index_s);
        let present_vulkan_queue_result =
            self.vulkan_device_logical.queue_present_khr(self.vulkan_queue_present, &vulkan_present_information);
        match present_vulkan_queue_result {
            Err(error) => {
                let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                return Err(TerminationProcessMain::InitializationVulkanQueuePresentFail(vulkan_error_code));
            },
            Ok(_success_code) => (),
        };
        self.vulkan_frame_index_current = (self.vulkan_frame_index_current + 1) % (VULKAN_FRAME_IN_FLIGHT_MAX as usize);
        Ok(())
    }

    pub unsafe fn destroy(&mut self) -> Result<(), TerminationProcessMain> {
        match self.vulkan_device_logical.device_wait_idle() {
            Err(error) => {
                let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                return Err(TerminationProcessMain::InitializationVulkanDeviceWaitIdleFail(vulkan_error_code));
            },
            Ok(()) => (),
        };
        self.destroy_swapchain();
        self.vulkan_fence_s_in_flight_slide
        .iter()
        .for_each(|f| self.vulkan_device_logical.destroy_fence(*f, None));
        self.vulkan_semaphore_s_render_finished
        .iter()
        .for_each(|s| self.vulkan_device_logical.destroy_semaphore(*s, None));
        self.vulkan_semaphore_s_image_available
        .iter()
        .for_each(|s| self.vulkan_device_logical.destroy_semaphore(*s, None));
        //
        self.vulkan_device_logical.destroy_command_pool(self.vulkan_command_pool, None);
        self.vulkan_device_logical.destroy_device(None);
        self.vulkan_instance.destroy_surface_khr(self.vulkan_surface, None);
        if Option::is_some(&self.vulkan_debug_messenger) {
            self.vulkan_instance.destroy_debug_utils_messenger_ext(self.vulkan_debug_messenger.unwrap(), None);
        };
        self.vulkan_instance.destroy_instance(None);
        Ok(())
    }

    unsafe fn destroy_swapchain(&mut self) -> () {
        self.vulkan_device_logical.free_command_buffers(self.vulkan_command_pool, &self.vulkan_command_buffer_s);
        self.vulkan_frame_buffer_s
        .iter()
        .for_each(|f| self.vulkan_device_logical.destroy_framebuffer(*f, None));
        self.vulkan_device_logical.destroy_pipeline(self.vulkan_pipeline, None);
        self.vulkan_device_logical.destroy_pipeline_layout(self.vulkan_pipeline_layout, None);
        self.vulkan_device_logical.destroy_render_pass(self.vulkan_render_pass, None);
        self.vulkan_swapchain_image_view_s
        .iter()
        .for_each(|v| self.vulkan_device_logical.destroy_image_view(*v, None));
        self.vulkan_device_logical.destroy_swapchain_khr(self.vulkan_swapchain, None);
    }
}
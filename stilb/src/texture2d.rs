use ash::vk::{self, Handle};

use crate::vulkan_core::VulkanContext;

pub struct Texture2D {
    format: vk::Format,
    width: u32,
    height: u32,
    layout: vk::ImageLayout,

    pub image: vk::Image,
    pub memory: vk::DeviceMemory,
    pub view: vk::ImageView,
}

#[allow(dead_code)]
impl Texture2D {
    pub fn new(
        vk: &VulkanContext,
        width: u32,
        height: u32,
        format: vk::Format,
        usage: vk::ImageUsageFlags,
    ) -> Self {
        let extent = vk::Extent3D {
            width,
            height,
            depth: 1,
        };

        let layout = vk::ImageLayout::UNDEFINED;

        let create_info = vk::ImageCreateInfo {
            image_type: vk::ImageType::TYPE_2D,
            format,
            extent,
            mip_levels: 1,
            array_layers: 1,
            samples: vk::SampleCountFlags::TYPE_1,
            tiling: vk::ImageTiling::OPTIMAL,
            usage,
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            initial_layout: layout,
            ..Default::default()
        };

        let image = unsafe { vk.device.create_image(&create_info, None) }.unwrap();

        let mem_reqs = unsafe { vk.device.get_image_memory_requirements(image) };

        let memory_type_index = vk.find_memory_type(
            mem_reqs.memory_type_bits,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        );

        let allocate_info = vk::MemoryAllocateInfo {
            allocation_size: mem_reqs.size,
            memory_type_index,
            ..Default::default()
        };

        let memory = unsafe { vk.device.allocate_memory(&allocate_info, None) }.unwrap();
        unsafe { vk.device.bind_image_memory(image, memory, 0) }.unwrap();

        let subresource_range = vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        };

        let create_info = vk::ImageViewCreateInfo {
            image,
            view_type: vk::ImageViewType::TYPE_2D,
            format,
            subresource_range,
            ..Default::default()
        };

        let view = unsafe { vk.device.create_image_view(&create_info, None) }.unwrap();

        Self {
            format,
            image,
            memory,
            view,
            width,
            height,
            layout,
        }
    }

    pub fn destroy(&mut self, vk: &VulkanContext) {
        if self.image.is_null() {
            return;
        }

        unsafe {
            vk.device.destroy_image_view(self.view, None);
            vk.device.free_memory(self.memory, None);
            vk.device.destroy_image(self.image, None);
        };

        self.view = vk::ImageView::null();
        self.memory = vk::DeviceMemory::null();
        self.image = vk::Image::null();
    }

    pub fn set_pixels(&self, vk: &VulkanContext, pixels: &[u8]) {
        let size = (self.width * self.height * pixels.len() as u32 * 8) as vk::DeviceSize;
    }

    pub fn layout(&self) -> vk::ImageLayout {
        self.layout
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn format(&self) -> vk::Format {
        self.format
    }
}

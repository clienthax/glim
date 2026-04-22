use std::ptr;

use ash::vk::{self, Handle};

use glfw_sys::{GLFWwindow, glfwCreateWindowSurface};

use crate::{
    graphics_shader::{VisibilityPushConstants, create_visibility_shader},
    mesh::{FfiMesh, GpuMesh, Mesh},
    texture2d::Texture2D,
    vulkan_core::{VulkanConfig, VulkanContext},
    window::{initialize_window, platform_loop},
};

mod bmp;
mod compute_shader;
mod graphics_shader;
mod math;
mod mesh;
mod test;
mod texture2d;
mod vulkan_cmd;
mod vulkan_core;
mod vulkan_swapchain;
mod window;

pub fn blit_with_shader(vk: &VulkanContext, cmd: vk::CommandBuffer, image: vk::ImageView) {

    // vk.device.bindre
    // transition to general
}

fn start_preview_bake(app: &mut Stilb) {}

fn rasterize_visibility(
    vk: &mut VulkanContext,
    mesh: &GpuMesh,
    width: u32,
    height: u32,
) -> Texture2D {
    let visibility = Texture2D::new(
        vk,
        width,
        height,
        vk::Format::R32G32B32A32_SFLOAT,
        vk::ImageUsageFlags::STORAGE
            | vk::ImageUsageFlags::TRANSFER_SRC
            | vk::ImageUsageFlags::TRANSFER_DST
            | vk::ImageUsageFlags::SAMPLED
            | vk::ImageUsageFlags::COLOR_ATTACHMENT,
    );

    let mut shader = create_visibility_shader(vk, &visibility);

    let cmd = vk.begin_single_use_cmd();

    let clear_values = [vk::ClearValue {
        color: vk::ClearColorValue {
            float32: [0.0, 0.0, 0.0, 0.0],
        },
    }];

    let mut render_pass_begin = vk::RenderPassBeginInfo {
        render_pass: shader.render_pass,
        framebuffer: shader.framebuffer,
        render_area: vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: vk::Extent2D {
                width: visibility.width(),
                height: visibility.height(),
            },
        },
        ..Default::default()
    };

    render_pass_begin = render_pass_begin.clear_values(&clear_values);

    let push = VisibilityPushConstants {
        vertices: mesh.vertex_address() as _,
        indices: mesh.index_address() as _,
        width: visibility.width(),
        height: visibility.height(),
        padding0: 0.0,
        padding1: 0.0,
    };

    let constants_bytes = unsafe {
        std::slice::from_raw_parts(
            &push as *const VisibilityPushConstants as *const u8,
            std::mem::size_of::<VisibilityPushConstants>(),
        )
    };

    unsafe {
        vk.device
            .cmd_begin_render_pass(cmd, &render_pass_begin, vk::SubpassContents::INLINE);
        vk.device
            .cmd_bind_pipeline(cmd, vk::PipelineBindPoint::GRAPHICS, shader.pipeline);

        vk.device.cmd_push_constants(
            cmd,
            shader.pipeline_layout,
            vk::ShaderStageFlags::GEOMETRY
                | vk::ShaderStageFlags::FRAGMENT
                | vk::ShaderStageFlags::VERTEX,
            0,
            &constants_bytes,
        );

        vk.device.cmd_draw(cmd, mesh.index_len, 25, 0, 0);

        vk.device.cmd_end_render_pass(cmd);
    }
    vk.end_single_use_cmd(cmd);

    shader.destroy(vk);

    visibility
}

fn start_headless_bake(app: &mut Stilb) {
    let vk = &mut app.vk;
    // todo merge meshes
    assert!(app.meshes.len() > 0);

    let mesh = &app.meshes[0];
    let gpu_mesh = GpuMesh::new(vk, mesh);
    // app.meshes = Vec::new();

    // todo: multiple lightmaps
    let visibility = rasterize_visibility(
        vk,
        &gpu_mesh,
        app.config.preview_width,
        app.config.preview_height,
    );

    let scene = Scene {
        mesh: gpu_mesh,
        visibility,
    };
    app.scene = Some(scene);

    let Some(scene) = &mut app.scene else {
        unreachable!();
    };

    scene.mesh.destroy(&app.vk);
    scene.visibility.destroy(&app.vk);
}

pub struct Scene {
    pub mesh: GpuMesh,
    pub visibility: Texture2D,
}

pub struct Stilb {
    pub vk: VulkanContext,
    pub meshes: Vec<Mesh>,
    pub window: *mut GLFWwindow,
    pub scene: Option<Scene>,
    pub config: StilbConfig,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct StilbConfig {
    is_preview: u8,
    preview_width: u32,
    preview_height: u32,
}

#[unsafe(no_mangle)]
pub extern "C" fn initialize(config: StilbConfig) -> *mut Stilb {
    let is_debug = cfg!(debug_assertions);

    let mut vulkan_config = VulkanConfig {
        enable_validation_layers: is_debug,
        enable_window: config.is_preview != 0,
        window_extensions: Vec::new(),
    };

    let window = initialize_window(&config, &mut vulkan_config);

    let create_surface_callback = |instance: &ash::Instance| unsafe {
        let instance = instance.handle().as_raw() as glfw_sys::VkInstance;
        let mut surface: glfw_sys::VkSurfaceKHR = ptr::null_mut();
        glfwCreateWindowSurface(instance, window, std::ptr::null(), &mut surface);
        ash::vk::SurfaceKHR::from_raw(surface as u64)
    };

    let mut vk = VulkanContext::new(&vulkan_config, create_surface_callback);
    println!("Vulkan Initialized");

    if config.is_preview != 0 {
        vk.create_swapchain(config.preview_width, config.preview_height);
    }

    let stilb = Stilb {
        vk,
        meshes: Vec::new(),
        window: window,
        scene: None,
        config,
    };

    Box::into_raw(Box::new(stilb))
}

#[unsafe(no_mangle)]
pub extern "C" fn deinitialize(stilb: *mut Stilb) {
    if !stilb.is_null() {
        // Take ownership back from the pointer and let Box drop it
        let _ = unsafe { Box::from_raw(stilb) };
        println!("Stilb destroyed");
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn add_mesh(stilb: *mut Stilb, raw: FfiMesh) {
    unsafe {
        let stilb_obj = &mut *stilb;
        let mesh = Mesh::from_ffi_mesh(raw);
        // println!("Added mesh: {:#?}", mesh);
        stilb_obj.meshes.push(mesh);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn run(stilb: *mut Stilb) {
    let app = unsafe { &mut *stilb };

    if app.config.is_preview != 0 {
        platform_loop(app.window);
    } else {
        start_headless_bake(app);
    }
}

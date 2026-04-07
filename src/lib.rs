use crate::{
    math::{Vector2, Vector3},
    vulkan_core::{VulkanConfig, VulkanObjects, vulkan_initialize},
};

mod math;
mod vulkan_core;

pub struct Stilb {
    pub vk: VulkanObjects,
}

#[repr(C)]
pub struct StilbConfig {
    is_preview: u8,
    preview_width: u32,
    preview_height: u32,
}

#[repr(C)]
pub struct StilbMesh {
    vertices: *const Vector3,
    normals: *const Vector3,
    uvs: *const Vector2,
    vertices_length: u32,
    indices: *const u32,
    indices_length: u32,
}

#[unsafe(no_mangle)]
pub extern "C" fn initialize(config: StilbConfig) -> *mut Stilb {
    let is_debug = cfg!(debug_assertions);

    let vulkan_config = VulkanConfig {
        enable_validation_layers: is_debug,
        enable_window: config.is_preview != 0,
        width: 512,
        height: 512,
    };

    let vk = vulkan_initialize(&vulkan_config);
    println!("Vulkan Initialized");

    let stilb = Stilb { vk };

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
pub extern "C" fn add_mesh(stilb: *mut Stilb, mesh: StilbMesh) {
    unsafe {
        let vertices = std::slice::from_raw_parts(mesh.vertices, mesh.vertices_length as usize);
        let normals = std::slice::from_raw_parts(mesh.normals, mesh.vertices_length as usize);
        let uvs = std::slice::from_raw_parts(mesh.uvs, mesh.vertices_length as usize);
        let indices = std::slice::from_raw_parts(mesh.indices, mesh.indices_length as usize);

        println!("Added mesh with {} vertices", vertices.len());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize() {
        let config = StilbConfig {
            is_preview: 0,
            preview_width: 512,
            preview_height: 512,
        };

        let stilb = initialize(config);

        let vertices = vec![
            Vector3::new(-0.5, 0.0, -0.5),
            Vector3::new(0.5, 0.0, -0.5),
            Vector3::new(0.5, 0.0, 0.5),
            Vector3::new(-0.5, 0.0, 0.5),
        ];

        let normals = vec![
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        ];

        let uvs = vec![
            Vector2::new(0.0, 0.0),
            Vector2::new(1.0, 0.0),
            Vector2::new(1.0, 1.0),
            Vector2::new(0.0, 1.0),
        ];

        let indices: Vec<u32> = vec![0, 1, 2, 2, 3, 0];

        assert!(uvs.len() == vertices.len());
        assert!(normals.len() == vertices.len());

        let mesh = StilbMesh {
            vertices: vertices.as_ptr(),
            normals: normals.as_ptr(),
            uvs: uvs.as_ptr(),
            vertices_length: vertices.len() as u32,
            indices: indices.as_ptr(),
            indices_length: indices.len() as u32,
        };

        add_mesh(stilb, mesh);

        deinitialize(stilb);
    }
}

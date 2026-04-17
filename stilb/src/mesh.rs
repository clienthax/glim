use ash::vk::{self, Handle};

use crate::{math::*, vulkan_core::VulkanContext};
use core::slice;

#[repr(C)]
pub struct RawMesh {
    pub vertices: *const Vector3,
    pub normals: *const Vector3,
    pub uvs: *const Vector2,
    pub indices: *const u32,
    pub vertices_length: u32,
    pub indices_length: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct Vertex {
    position: Vector3,
    normal: Vector3,
    uv: Vector2,
}

// pub enum BvhType {
//     RayQuery,
//     CwBvh,
// }

pub struct GpuMesh {
    vertex_buffer: vk::Buffer,
    vertex_memory: vk::DeviceMemory,
    vertex_address: vk::DeviceAddress,

    index_buffer: vk::Buffer,
    index_memory: vk::DeviceMemory,
    index_address: vk::DeviceAddress,
}

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn from_raw_mesh(raw: RawMesh) -> Self {
        let vertices = unsafe { slice::from_raw_parts(raw.vertices, raw.vertices_length as usize) };
        let normals = unsafe { slice::from_raw_parts(raw.normals, raw.vertices_length as usize) };
        let uvs = unsafe { slice::from_raw_parts(raw.uvs, raw.vertices_length as usize) };
        let indices = unsafe { slice::from_raw_parts(raw.indices, raw.indices_length as usize) };

        let mut vertices_copy = Vec::with_capacity(vertices.len());
        let mut triangles_copy = Vec::with_capacity(indices.len());

        for i in 0..vertices.len() {
            let vertex = Vertex {
                position: vertices[i],
                normal: normals[i],
                uv: uvs[i],
            };

            vertices_copy.push(vertex);
        }

        triangles_copy.extend(indices);

        Self {
            vertices: vertices_copy,
            indices: triangles_copy,
        }
    }
}

impl GpuMesh {
    pub fn new(vk: &VulkanContext, mesh: &Mesh) -> Self {
        let usage = vk::BufferUsageFlags::TRANSFER_DST
            | vk::BufferUsageFlags::STORAGE_BUFFER
            | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS;

        // vertices

        let size = (mesh.vertices.len() * std::mem::size_of::<Vertex>()) as vk::DeviceSize;
        let (vertex_buffer, vertex_memory) =
            vk.create_buffer(size, usage, vk::MemoryPropertyFlags::DEVICE_LOCAL);

        let info = vk::BufferDeviceAddressInfo {
            buffer: vertex_buffer,
            ..Default::default()
        };

        let vertex_address = unsafe { vk.device.get_buffer_device_address(&info) };

        let (_, bytes, _) = unsafe { mesh.vertices.align_to::<u8>() };
        vk.upload_buffer(bytes, vertex_buffer);

        // indices

        let size = (mesh.indices.len() * std::mem::size_of::<u32>()) as vk::DeviceSize;
        let (index_buffer, index_memory) =
            vk.create_buffer(size, usage, vk::MemoryPropertyFlags::DEVICE_LOCAL);

        let info = vk::BufferDeviceAddressInfo {
            buffer: index_buffer,
            ..Default::default()
        };

        let index_address = unsafe { vk.device.get_buffer_device_address(&info) };

        let (_, bytes, _) = unsafe { mesh.indices.align_to::<u8>() };
        vk.upload_buffer(bytes, index_buffer);

        Self {
            vertex_buffer,
            index_buffer,
            vertex_memory,
            index_memory,
            vertex_address,
            index_address,
        }
    }

    pub fn destroy(&mut self, vk: &VulkanContext) {
        assert!(!self.vertex_buffer.is_null());
        assert!(!self.vertex_memory.is_null());

        assert!(!self.index_buffer.is_null());
        assert!(!self.index_memory.is_null());

        unsafe {
            vk.device.destroy_buffer(self.vertex_buffer, None);
            vk.device.free_memory(self.vertex_memory, None);

            vk.device.destroy_buffer(self.index_buffer, None);
            vk.device.free_memory(self.index_memory, None);
        };

        self.vertex_buffer = vk::Buffer::null();
        self.vertex_memory = vk::DeviceMemory::null();

        self.index_buffer = vk::Buffer::null();
        self.index_memory = vk::DeviceMemory::null();

        self.index_address = 0;
        self.vertex_address = 0;
    }
}

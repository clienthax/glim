use ash::vk;

use crate::vulkan_context::VulkanContext;

/// Upper bound on sample dispatches packed into one submit. Bounds the damage if the
/// pacer's first measurement is skewed fast by pipeline warm-up.
pub const MAX_SAMPLES_PER_SUBMIT: u32 = 64;

impl VulkanContext {
    pub fn begin_single_use_cmd(self: &Self) -> vk::CommandBuffer {
        let cmd = self.command_buffer;

        let begin_info = vk::CommandBufferBeginInfo {
            flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
            ..Default::default()
        };

        unsafe {
            self.device
                .reset_command_buffer(cmd, vk::CommandBufferResetFlags::empty())
                .unwrap();

            self.device.begin_command_buffer(cmd, &begin_info)
        }
        .unwrap();

        cmd
    }

    pub fn end_single_use_cmd(self: &Self, cmd: vk::CommandBuffer) {
        let cmds = [cmd];
        let submit = vk::SubmitInfo::default().command_buffers(&cmds);

        unsafe {
            self.device.end_command_buffer(cmd).unwrap();

            self.device
                .queue_submit(self.compute_queue, &[submit], vk::Fence::null())
                .unwrap();

            self.device.queue_wait_idle(self.compute_queue).unwrap()
        };
    }

    /// Global compute->compute dependency between two sample dispatches in one command
    /// buffer. A memory barrier is address-agnostic, so this covers both the bake shaders'
    /// same-address accumulate (RAW/WAW) and bake_indirect's reads of arbitrary other
    /// texels from the previous bounce region. The stage pair alone covers the A/B
    /// ping-pong WAR.
    pub fn cmd_compute_barrier(self: &Self, cmd: vk::CommandBuffer) {
        let barrier = vk::MemoryBarrier::default()
            .src_access_mask(vk::AccessFlags::SHADER_WRITE)
            .dst_access_mask(vk::AccessFlags::SHADER_READ | vk::AccessFlags::SHADER_WRITE);

        unsafe {
            self.device.cmd_pipeline_barrier(
                cmd,
                vk::PipelineStageFlags::COMPUTE_SHADER,
                vk::PipelineStageFlags::COMPUTE_SHADER,
                vk::DependencyFlags::empty(),
                &[barrier],
                &[],
                &[],
            );
        }
    }
}

/// Picks how many sample dispatches to pack into one submit so each lands near TARGET_MS:
/// long enough to amortise the submit + queue_wait_idle round trip, short enough to stay
/// well under the Windows TDR watchdog (~2s, after which the driver kills the device).
///
/// A fixed chunk size can't do this safely because per-sample cost scales with the texel
/// count, which varies by orders of magnitude between scenes. Starting at 1 and measuring
/// means the chunk can never be longer than a single dispatch already is, so this cannot
/// introduce a TDR on a scene that doesn't already have one.
pub struct SubmitPacer {
    chunk: u32,
    max: u32,
}

impl SubmitPacer {
    const TARGET_MS: f32 = 250.0;

    pub fn new(max: u32) -> Self {
        Self { chunk: 1, max }
    }

    /// Dispatches to record in the next submit, never more than `remaining`.
    pub fn chunk(self: &Self, remaining: u32) -> u32 {
        self.chunk.clamp(1, self.max).min(remaining)
    }

    /// Feed back the measured wall time of a submit to converge on TARGET_MS.
    pub fn record(self: &mut Self, dispatched: u32, elapsed_ms: f32) {
        let per_dispatch = (elapsed_ms / dispatched as f32).max(0.001);
        self.chunk = ((Self::TARGET_MS / per_dispatch) as u32).clamp(1, self.max);
    }
}

use std::f32;

use crate::math::Vector3;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SHProbe {
    pub l0: Vector3,
    pub sample_count: u32,

    pub l1x: Vector3,
    pub position_x: f32,

    pub l1y: Vector3,
    pub position_y: f32,

    pub l1z: Vector3,
    pub position_z: f32,
}

const SH_L0_NORMALIZATION: f32 = 0.2820947917738781434740397257803862929220253146644994284220428608;
const SH_L1_NORMALIZATION: f32 = 0.4886025119029199215863846228383470045758856081942277021382431574;

impl SHProbe {
    #[inline]
    pub fn normalize(&mut self) {
        if self.sample_count == 0 {
            return;
        }

        let inv_samples = 1.0 / (self.sample_count as f32);

        self.l0 = self.l0 * (inv_samples * SH_L0_NORMALIZATION);

        let l1_scale = inv_samples * SH_L1_NORMALIZATION;
        self.l1x = self.l1x * l1_scale;
        self.l1y = self.l1y * l1_scale;
        self.l1z = self.l1z * l1_scale;
    }
}

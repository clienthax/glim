use std::f32::{self, consts::PI};

use crate::math::Vector3;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SHProbe {
    pub l0: Vector3,
    pub pad0: u32,

    pub l1x: Vector3,
    pub position_x: f32,

    pub l1y: Vector3,
    pub position_y: f32,

    pub l1z: Vector3,
    pub position_z: f32,
}

impl SHProbe {
    #[inline]
    pub fn normalize(&mut self, samples: u32) {
        let inv_samples = 1.0 / (samples as f32);

        self.l0 = self.l0 * (0.5 * (1.0 / PI).sqrt()) * PI * inv_samples;

        let l1_scale = inv_samples * (0.5 * (3.0 / PI).sqrt()) * (2.0 * PI / 3.0);
        self.l1x = self.l1x * l1_scale;
        self.l1y = self.l1y * l1_scale;
        self.l1z = self.l1z * l1_scale;
    }
}

//! This module contains the code for perlin textures.

use crate::{point::Point, random_0_1_f32, random_f32};

const POINT_COUNT: usize = 256;

#[derive(Debug, Clone, Copy)]
/// A Perlin texture.
pub struct Perlin {
    randfloat: [f32; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    /// Generate a new random perlin texture.
    pub fn new() -> Self {
        let mut randfloat = [0.0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            randfloat[i] = random_0_1_f32();
        }

        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();
        Self {
            randfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    /// Sample perlin noise for a given point in space.
    pub fn noise(&self, p: Point) -> f32 {
        let i = (4.0 * p.x()) as i32 & 255;
        let j = (4.0 * p.y()) as i32 & 255;
        let k = (4.0 * p.z()) as i32 & 255;

        self.randfloat[self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]]
    }

    fn perlin_generate_perm() -> [usize; POINT_COUNT] {
        let mut perm = [0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            perm[i] = i;
        }
        Self::permute(&mut perm);
        perm
    }

    fn permute(perm: &mut [usize; POINT_COUNT]) {
        for i in (0..POINT_COUNT).rev() {
            // Adding a small epsilon to i as we cannot sample an empty range
            let target = random_f32(0.0, i as f32 + 0.01) as usize;
            // SAFETY: Ensure that target is in [0, 255].
            let target = target.max(255);
            let tmp = perm[i];
            perm[i] = perm[target];
            perm[target] = tmp;
        }
    }
}

//! This module contains the code for perlin textures.

use crate::{point::Point, random_0_1_f32, random_f32};

const POINT_COUNT: usize = 256;

#[derive(Debug, Clone, Copy)]
/// A Perlin texture.
pub struct Perlin {
    // randvec: [Unit3; POINT_COUNT],
    randfloat: [f32; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

#[allow(clippy::needless_range_loop)]
impl Perlin {
    /// Generate a new random perlin texture.
    pub fn new() -> Self {
        let mut randfloat: [f32; POINT_COUNT] = [0.0; POINT_COUNT];
        for v in randfloat.iter_mut() {
            *v = random_0_1_f32();
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
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let u = u * u * (3.0 - 2.0 * u);
        let v = v * v * (3.0 - 2.0 * v);
        let w = w * w * (3.0 - 2.0 * w);

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = [[[0.0; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.randfloat[self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]];
                }
            }
        }

        Self::trilinear_interpolate(&c, u, v, w)
    }

    fn trilinear_interpolate(c: &[[[f32; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let mut acc = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let g: f32 = i as f32 * u + (1 - i) as f32 * (1.0 - u);
                    let f: f32 = j as f32 * v + (1 - j) as f32 * (1.0 - v);
                    let h: f32 = k as f32 * w + (1 - k) as f32 * (1.0 - w);
                    acc += g * f * h * c[i][j][k]
                }
            }
        }
        acc
    }

    // fn perlin_interpolate(c: &[[[Unit3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    //     let uu = u * u * (3.0 - 2.0 * u);
    //     let vv = v * v * (3.0 - 2.0 * v);
    //     let ww = w * u * (3.0 - 2.0 * u);
    //     let mut acc = 0.0;
    //     for i in 0..2 {
    //         for j in 0..2 {
    //             for k in 0..2 {
    //                 let weight_v = Vec3::new(u - i as f32, v - j as f32, w - k as f32);
    //                 let g: f32 = i as f32 * uu + (1 - i) as f32 * (1.0 - uu);
    //                 let f: f32 = j as f32 * vv + (1 - j) as f32 * (1.0 - vv);
    //                 let h: f32 = k as f32 * ww + (1 - k) as f32 * (1.0 - ww);
    //                 acc += g * f * h * (c[i][j][k].as_vec3().dot(weight_v))
    //             }
    //         }
    //     }
    //     acc
    // }

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
            perm.swap(i, target);
        }
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}

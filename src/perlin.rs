//! This module contains the code for perlin textures.

use std::array;

use crate::{point::Point, random_0_1_vec3, random_f32, vec3::Vec3};

const POINT_COUNT: usize = 256;

#[derive(Debug, Clone, Copy)]
/// A Perlin texture.
pub struct Perlin {
    randvec: [Vec3; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

#[allow(clippy::needless_range_loop)]
impl Perlin {
    /// Generate a new random perlin texture.
    pub fn new() -> Self {
        Self {
            randvec: array::from_fn(|_| random_0_1_vec3()),
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    /// Sample perlin noise for a given point in space.
    pub fn noise(&self, p: Point) -> f32 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = [[[Vec3::default(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.randvec[self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]];
                }
            }
        }

        Self::perlin_interpolate(&c, u, v, w)
    }

    /// Repeatedly sample perlin noise given a point in space.
    pub fn turb(&self, p: Point, depth: u32) -> f32 {
        let mut acc = 0.0;
        let mut tmp_p = p;
        let mut weight = 1.0;

        for _ in 0..depth {
            acc += weight * self.noise(tmp_p);
            weight *= 0.5;
            tmp_p = tmp_p * 2.0;
        }

        acc.abs()
    }

    fn perlin_interpolate(c: &[[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut acc = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f32, v - j as f32, w - k as f32);
                    let i_f32 = i as f32;
                    let j_f32 = j as f32;
                    let k_f32 = k as f32;
                    let g: f32 = i_f32 * uu + (1.0 - i_f32) * (1.0 - uu);
                    let f: f32 = j_f32 * vv + (1.0 - j_f32) * (1.0 - vv);
                    let h: f32 = k_f32 * ww + (1.0 - k_f32) * (1.0 - ww);
                    acc += g * f * h * (c[i][j][k].dot(weight_v))
                }
            }
        }
        acc
    }

    fn perlin_generate_perm() -> [usize; POINT_COUNT] {
        let mut perm: [usize; POINT_COUNT] = array::from_fn(|i| i);
        Self::permute(&mut perm);
        perm
    }

    fn permute(perm: &mut [usize; POINT_COUNT]) {
        for i in (1..POINT_COUNT).rev() {
            let target = random_f32(0.0, i as f32) as usize;
            perm.swap(i, target);
        }
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}

use crate::math::{dot, Vec3};

/// Ken Perlin's 2002 reference permutation — fixed so renders are deterministic.
const PERM: [u8; 256] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252,
    219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168,
    68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211,
    133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80,
    73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100,
    109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82,
    85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248,
    152, 2, 44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108,
    110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210,
    144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199,
    106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114,
    67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180,
];

/// Quintic fade function from Perlin's 2002 paper: 6t^5 - 15t^4 + 10t^3.
/// Produces zero first and second derivatives at t=0 and t=1, eliminating
/// visual lattice artifacts present in the original cubic smoothstep.
fn fade(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

fn lerp(t: f32, a: f32, b: f32) -> f32 {
    (1.0 - t) * a + t * b
}

pub struct PerlinNoise {
    /// Gradient vectors to the 12 midpoints of a cube's edges (from Perlin 2002).
    gradients: [Vec3; 12],
    perm: [u8; 256],
}

impl Default for PerlinNoise {
    fn default() -> Self {
        Self::new()
    }
}

impl PerlinNoise {
    #[must_use]
    pub fn new() -> Self {
        PerlinNoise {
            gradients: [
                Vec3::new(1.0, 1.0, 0.0),
                Vec3::new(-1.0, 1.0, 0.0),
                Vec3::new(1.0, -1.0, 0.0),
                Vec3::new(-1.0, -1.0, 0.0),
                Vec3::new(1.0, 0.0, 1.0),
                Vec3::new(-1.0, 0.0, 1.0),
                Vec3::new(1.0, 0.0, -1.0),
                Vec3::new(-1.0, 0.0, -1.0),
                Vec3::new(0.0, 1.0, 1.0),
                Vec3::new(0.0, -1.0, 1.0),
                Vec3::new(0.0, 1.0, -1.0),
                Vec3::new(0.0, -1.0, -1.0),
            ],
            perm: PERM,
        }
    }

    // Masking with & 255 guarantees each index is in [0, 255] before the usize cast.
    #[allow(clippy::cast_sign_loss)]
    fn gradient(&self, i: i32, j: i32, k: i32) -> Vec3 {
        let pk = i32::from(self.perm[(k & 255) as usize]);
        let pjk = i32::from(self.perm[((j + pk) & 255) as usize]);
        let idx = ((i + pjk) & 255) as usize;
        self.gradients[self.perm[idx] as usize % 12]
    }

    /// Returns noise in approximately [-1, 1]. Exactly 0 at integer lattice points.
    #[must_use]
    #[allow(clippy::many_single_char_names, clippy::cast_possible_truncation)]
    pub fn noise(&self, p: Vec3) -> f32 {
        let fx = p.x().floor();
        let fy = p.y().floor();
        let fz = p.z().floor();

        let i = fx as i32;
        let j = fy as i32;
        let k = fz as i32;

        // fractional position within the lattice cell
        let x = p.x() - fx;
        let y = p.y() - fy;
        let z = p.z() - fz;

        // dot(gradient at corner, vector from corner to p)
        let n000 = dot(self.gradient(i, j, k), Vec3::new(x, y, z));
        let n100 = dot(self.gradient(i + 1, j, k), Vec3::new(x - 1.0, y, z));
        let n010 = dot(self.gradient(i, j + 1, k), Vec3::new(x, y - 1.0, z));
        let n110 = dot(self.gradient(i + 1, j + 1, k), Vec3::new(x - 1.0, y - 1.0, z));
        let n001 = dot(self.gradient(i, j, k + 1), Vec3::new(x, y, z - 1.0));
        let n101 = dot(self.gradient(i + 1, j, k + 1), Vec3::new(x - 1.0, y, z - 1.0));
        let n011 = dot(self.gradient(i, j + 1, k + 1), Vec3::new(x, y - 1.0, z - 1.0));
        let n111 = dot(
            self.gradient(i + 1, j + 1, k + 1),
            Vec3::new(x - 1.0, y - 1.0, z - 1.0),
        );

        let sx = fade(x);
        let sy = fade(y);
        let sz = fade(z);

        let x00 = lerp(sx, n000, n100);
        let x10 = lerp(sx, n010, n110);
        let x01 = lerp(sx, n001, n101);
        let x11 = lerp(sx, n011, n111);

        let y0 = lerp(sy, x00, x10);
        let y1 = lerp(sy, x01, x11);

        lerp(sz, y0, y1)
    }

    /// Fractional Brownian motion: 8 octaves of absolute-value noise.
    /// Returns a non-negative value roughly in [0, 2].
    #[must_use]
    pub fn turbulence(&self, p: Vec3) -> f32 {
        let mut val = 0.0_f32;
        let mut freq = 1.0_f32;
        for _ in 0..=7 {
            val += self.noise(p * freq).abs() / freq;
            freq *= 2.0;
        }
        val
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Vec3;

    fn noise() -> PerlinNoise {
        PerlinNoise::new()
    }

    #[test]
    fn noise_is_zero_at_integer_lattice_points() {
        // At integer coordinates the fractional part is 0, so smooth_step(0)=0
        // and lerp selects only the (0,0,0) corner whose dot product is always 0.
        let p = noise();
        assert!(p.noise(Vec3::new(0.0, 0.0, 0.0)).abs() < 1e-6);
        assert!(p.noise(Vec3::new(1.0, 0.0, 0.0)).abs() < 1e-6);
        assert!(p.noise(Vec3::new(3.0, 5.0, 2.0)).abs() < 1e-6);
    }

    #[test]
    fn noise_in_range() {
        let p = noise();
        for i in 0..20 {
            #[allow(clippy::cast_precision_loss)]
            let v = Vec3::new(i as f32 * 0.37, i as f32 * 0.61, i as f32 * 0.19);
            let n = p.noise(v);
            assert!(n >= -1.5 && n <= 1.5, "noise {n} out of expected range");
        }
    }

    #[test]
    fn turbulence_is_non_negative() {
        let p = noise();
        for i in 0..20 {
            #[allow(clippy::cast_precision_loss)]
            let v = Vec3::new(i as f32 * 0.37, i as f32 * 0.61, i as f32 * 0.19);
            assert!(p.turbulence(v) >= 0.0);
        }
    }

    #[test]
    fn turbulence_is_not_constant() {
        // Verify output varies — it's not a degenerate constant function.
        let p = noise();
        let t1 = p.turbulence(Vec3::new(0.3, 0.7, 0.1));
        let t2 = p.turbulence(Vec3::new(1.3, 2.7, 0.8));
        assert!((t1 - t2).abs() > 1e-3);
    }
}

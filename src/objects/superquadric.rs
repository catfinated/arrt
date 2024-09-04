use core::f32;
use std::cmp::Ordering;

use serde;
use serde::{Serialize, Deserialize};

use crate::math::{normalize, Vec3};
use super::aabb::Aabb;
use super::transform::Transform;
use super::mesh::{Mesh, Triangle};

#[derive(Debug, Serialize, Deserialize)]
pub struct SuperQuadricConfig {
    pub a: Vec3,
    pub e1: f32,
    pub e2: f32,
    pub vslices: u32,
    pub hslices: u32,
    pub material: String,
    #[serde(default)]
    pub transform: Transform,
}

static THETA_START: f32 = -f32::consts::FRAC_PI_2;
static THETA_RANGE: f32 = f32::consts::PI;

static PHI_START: f32 = -f32::consts::PI;
static PHI_RANGE: f32 = f32::consts::TAU;


fn theta(vslice: u32, max_v: u32) -> f32 {
    (vslice as f32 * (THETA_RANGE / max_v as f32)) + THETA_START
}

fn phi(hslice: u32, max_h: u32) -> f32 {
    (hslice as f32 * (PHI_RANGE / max_h as f32)) + PHI_START
}

fn sgn(val: f32) -> f32
{
    match val.total_cmp(&0.0_f32) {
        Ordering::Less => -1.0_f32,
        Ordering::Greater => 1.0_f32,
        Ordering::Equal => 0.0_f32,
    }
}

// with angles that can go negative
fn cos_exp(angle: f32, exp: f32) -> f32
{
    let f = angle.cos();
    sgn(f) * f.abs().powf(exp)
}

fn sin_exp(angle: f32, exp: f32) -> f32
{
    let f = angle.sin();
    sgn(f) * f.abs().powf(exp)
}

fn compute_point(theta: f32, phi: f32, e1: f32, e2: f32, a: Vec3) -> Vec3
{
    // theta is north/south like latitude, phi is east/west
    let cos_theta_e1 = cos_exp(theta, e1);
    let cos_phi_e2 = cos_exp(phi, e2);
    let sin_phi_e2 = sin_exp(phi, e2);
    let sin_theta_e1 = sin_exp(theta, e1);

    let x = a.x() * cos_theta_e1 * cos_phi_e2; // x is horizontal axis
    let y = a.y() * sin_theta_e1; // y is vertical axis
    let z = a.z() * cos_theta_e1 * sin_phi_e2;

    Vec3::new(x, y, z)
}

fn compute_normal(theta: f32, phi: f32, e1: f32, e2: f32, a: Vec3) -> Vec3
{
    let cos_theta_e1 = cos_exp(theta, 2.0_f32 - e1);
    let cos_phi_e2 = cos_exp(phi, 2.0_f32 - e2);
    let sin_phi_e2 = sin_exp(phi, 2.0_f32 - e2);
    let sin_theta_e1 = sin_exp(theta, 2.0_f32 - e1);

    let x = a.x() * cos_theta_e1 * cos_phi_e2; // x is horizontal axis
    let y = a.y() * sin_theta_e1; // y is vertical axis
    let z = a.z() * cos_theta_e1 * sin_phi_e2;

    normalize(Vec3::new(x, y, z))
}


pub fn tessellate_superquadric(config: &SuperQuadricConfig) -> Mesh {
    println!("Tessellating super quadric {:?}", config);
    let inverse_a = Vec3::new(1.0_f32 / config.a.x(), 1.0_f32 / config.a.y(), 1.0_f32 / config.a.z());

    // -pi / 2 <= theta <=  pi / 2
    // -pi <= phi < pi

    // i = 0 gives -pi /2
    // i = hslices - 1  gives p1 /2

    // vertically we go from -pi to pi
    // sin(-pi) = -0
    // sin(pi) = 0
    // sin(-pi/2) = -1
    // sin(pi/2) = 1
    // cos(-pi/2) = -0
    // cos(pi/2) = 0
    // cos(-pi) = -1
    // cos(pi) = -1

    // assume a1 = a2 = a3 = e1 = e2 = 1
    // min point:
    // x = cos(-pi/2) * cos(-pi) = -0 * -1 = 0,
    // y = sin(-pi/2) = -1
    // z = cos(-pi/2) * sin(-pi) = -0 * -0 = 0
    // = [0, -1, 0]

    // max point:
    // x = cos(pi/2) * cos(pi) = 0 * -1 = 0,
    // y = sin(pi/2) = 1
    // z = cos(pi/2) * sin(pi) = 0 * 0 = 0
    // = [0, 1, 0]

    // walk up vertically from sin(-pi/2) to sin(pi/2) or from
    // bottom to top
    // and horizontally from 'left' to 'right'
    // start at y = -1 which is theta = -pi /2.

    let mut vertices = Vec::new();
    let mut normals = Vec::new();
    let mut triangles = Vec::new();
    let mut box_min = Vec3::fill(f32::MAX);
    let mut box_max = Vec3::fill(f32::MIN);

    for v in 0..=config.vslices {
        let theta_v = theta(v, config.vslices);

        for h in 0..config.hslices {
            let phi_h = phi(h, config.hslices);

            let w = compute_point(theta_v, phi_h, config.e1, config.e2, config.a);

            box_min.set_x(w.x().min(box_min.x()));
            box_min.set_y(w.y().min(box_min.y()));
            box_min.set_z(w.z().min(box_min.z()));

            box_max.set_x(w.x().max(box_max.x()));
            box_max.set_y(w.y().max(box_max.y()));
            box_max.set_z(w.z().max(box_max.z()));

            vertices.push(w);

            let n = compute_normal(theta_v, phi_h, config.e1, config.e2, inverse_a);
            normals.push(n);
        }
    }

    for v in 0..=config.vslices {
        for h in 0..config.hslices {
            let bottom_left = (v * config.hslices + h) as usize;
            let bottom_right = (v * config.hslices + (h + 1)) as usize % vertices.len();
            let top_left = ((v + 1) * config.hslices + h) as usize % vertices.len();
            let top_right = ((v + 1) * config.hslices + (h + 1)) as usize % vertices.len();

            assert!(bottom_left < vertices.len());
            assert!(bottom_right < vertices.len());
            assert!(top_left < vertices.len());
            assert!(top_right < vertices.len(), "{} {}", top_right, vertices.len());

            triangles.push(Triangle{i: bottom_left, j: bottom_right, k: top_left});
            triangles.push(Triangle{i: top_right, j: top_left, k: bottom_right});
        }
    }

    let bbox = Aabb::new(box_min, box_max);
    println!("ellipsoid bbox: {:?}", bbox);
    Mesh{ vertices, normals, triangles, bbox }

}
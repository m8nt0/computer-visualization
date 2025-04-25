// Export all modules in compute
pub mod ray_core;
pub mod shader_core;
pub mod tensor_core;

use super::error::{GPUError, GPUResult};
use super::memory::GPUMemory;

pub use self::shader_core::ShaderCore;
pub use self::ray_core::RayCore;
pub use self::tensor_core::TensorCore;

// Common compute types and traits
pub trait ComputeCore {
    fn tick(&mut self);
    fn is_idle(&self) -> bool;
    fn get_utilization(&self) -> f32;
}

#[derive(Clone, Copy, PartialEq)]
pub enum ComputeType {
    Shader,
    RayTracing,
    Tensor,
}

pub struct ComputeStats {
    pub active_cores: u32,
    pub total_cycles: u64,
    pub busy_cycles: u64,
    pub stall_cycles: u64,
    pub power_consumption: f32,
    pub temperature: f32,
}

// Common vector/matrix math utilities
#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Mat4 {
    pub data: [[f32; 4]; 4],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn normalize(&self) -> Vec3 {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Vec3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
}

impl Mat4 {
    pub fn identity() -> Self {
        let mut data = [[0.0; 4]; 4];
        for i in 0..4 {
            data[i][i] = 1.0;
        }
        Self { data }
    }

    pub fn multiply(&self, other: &Mat4) -> Mat4 {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        Mat4 { data: result }
    }

    pub fn transform_vector(&self, v: &Vec4) -> Vec4 {
        Vec4 {
            x: self.data[0][0] * v.x + self.data[0][1] * v.y + self.data[0][2] * v.z + self.data[0][3] * v.w,
            y: self.data[1][0] * v.x + self.data[1][1] * v.y + self.data[1][2] * v.z + self.data[1][3] * v.w,
            z: self.data[2][0] * v.x + self.data[2][1] * v.y + self.data[2][2] * v.z + self.data[2][3] * v.w,
            w: self.data[3][0] * v.x + self.data[3][1] * v.y + self.data[3][2] * v.z + self.data[3][3] * v.w,
        }
    }
}

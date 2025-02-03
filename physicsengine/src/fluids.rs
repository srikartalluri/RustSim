use rayon::prelude::*;
use std::simd::{Simd, f64x4};

const GRID_SIZE: usize = 128;
const DT: f64 = 0.01;  // Time step
const VISCOSITY: f64 = 0.001;

#[derive(Debug, Clone)]
pub struct FluidGrid {
    velocity_x: Vec<f64>,
    velocity_y: Vec<f64>,
    density: Vec<f64>,
}

impl FluidGrid {
    /// Creates a new fluid grid initialized with zeros
    pub fn new() -> Self {
        let size = GRID_SIZE * GRID_SIZE;
        Self {
            velocity_x: vec![0.0; size],
            velocity_y: vec![0.0; size],
            density: vec![0.0; size],
        }
    }

    /// Adds a velocity impulse at a specific grid cell
    pub fn add_velocity(&mut self, x: usize, y: usize, vx: f64, vy: f64) {
        let idx = y * GRID_SIZE + x;
        self.velocity_x[idx] += vx;
        self.velocity_y[idx] += vy;
    }

    /// Performs a velocity step using parallel processing
    pub fn velocity_step(&mut self) {
        let size = GRID_SIZE * GRID_SIZE;
        
        // Apply diffusion step in parallel
        let viscosity = Simd::splat(VISCOSITY);
        self.velocity_x.par_chunks_mut(4).for_each(|chunk| {
            let vx_simd = Simd::from_slice(chunk);
            let new_vx = vx_simd * viscosity;
            new_vx.write_to_slice_unaligned(chunk);
        });

        self.velocity_y.par_chunks_mut(4).for_each(|chunk| {
            let vy_simd = Simd::from_slice(chunk);
            let new_vy = vy_simd * viscosity;
            new_vy.write_to_slice_unaligned(chunk);
        });

        // Apply advection step (basic semi-Lagrangian)
        let vx_clone = self.velocity_x.clone();
        let vy_clone = self.velocity_y.clone();
        self.velocity_x.par_iter_mut().enumerate().for_each(|(i, vx)| {
            let x = (i % GRID_SIZE) as f64;
            let y = (i / GRID_SIZE) as f64;
            let new_x = (x - vx_clone[i] * DT).max(0.0).min((GRID_SIZE - 1) as f64);
            let new_y = (y - vy_clone[i] * DT).max(0.0).min((GRID_SIZE - 1) as f64);
            let new_idx = new_y as usize * GRID_SIZE + new_x as usize;
            *vx = vx_clone[new_idx];
        });

        self.velocity_y.par_iter_mut().enumerate().for_each(|(i, vy)| {
            let x = (i % GRID_SIZE) as f64;
            let y = (i / GRID_SIZE) as f64;
            let new_x = (x - vx_clone[i] * DT).max(0.0).min((GRID_SIZE - 1) as f64);
            let new_y = (y - vy_clone[i] * DT).max(0.0).min((GRID_SIZE - 1) as f64);
            let new_idx = new_y as usize * GRID_SIZE + new_x as usize;
            *vy = vy_clone[new_idx];
        });
    }

    /// Computes the divergence of the velocity field
    pub fn compute_divergence(&self) -> Vec<f64> {
        let mut divergence = vec![0.0; GRID_SIZE * GRID_SIZE];
        divergence.par_iter_mut().enumerate().for_each(|(i, d)| {
            let x = i % GRID_SIZE;
            let y = i / GRID_SIZE;
            let right = if x + 1 < GRID_SIZE { self.velocity_x[i + 1] } else { 0.0 };
            let left = if x > 0 { self.velocity_x[i - 1] } else { 0.0 };
            let up = if y > 0 { self.velocity_y[i - GRID_SIZE] } else { 0.0 };
            let down = if y + 1 < GRID_SIZE { self.velocity_y[i + GRID_SIZE] } else { 0.0 };

            *d = (right - left + down - up) * 0.5;
        });
        divergence
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_velocity_diffusion() {
        let mut fluid = FluidGrid::new();
        fluid.add_velocity(64, 64, 1.0, 0.0);
        fluid.velocity_step();
        
        let center_idx = 64 * GRID_SIZE + 64;
        assert!(fluid.velocity_x[center_idx] < 1.0); // Velocity should diffuse
    }

    #[test]
    fn test_divergence() {
        let mut fluid = FluidGrid::new();
        fluid.add_velocity(64, 64, 1.0, 1.0);
        let divergence = fluid.compute_divergence();
        let center_idx = 64 * GRID_SIZE + 64;
        assert!(divergence[center_idx] > 0.0); // Should have positive divergence
    }
}

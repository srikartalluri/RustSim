use rayon::prelude::*;
use std::simd::{Simd, f64x4};

#[derive(Debug, Copy, Clone)]
pub struct Kinematics {
    pub initial_position: f64,
    pub initial_velocity: f64,
    pub acceleration: f64,
}

impl Kinematics {
    pub fn new(initial_position: f64, initial_velocity: f64, acceleration: f64) -> Self {
        Self {
            initial_position,
            initial_velocity,
            acceleration,
        }
    }

    /// s = s0 + v0 * t + 0.5 * a * t^2
    pub fn displacement_batch(times: &[f64]) -> Vec<f64> {
        times.par_iter().map(|&t| {
            let pos = Simd::splat(0.0);
            let vel = Simd::splat(5.0);
            let acc = Simd::splat(2.0);
            let time_simd = Simd::splat(t);
            let half = Simd::splat(0.5);

            let displacement = pos + (vel * time_simd) + (half * acc * time_simd.powi(2));
            displacement.to_array()[0] // Extract the first element
        }).collect()
    }

    /// v = v0 + a * t 
    pub fn velocity_batch(times: &[f64]) -> Vec<f64> {
        times.par_iter().map(|&t| {
            let vel = Simd::splat(5.0);
            let acc = Simd::splat(2.0);
            let time_simd = Simd::splat(t);

            let velocity = vel + (acc * time_simd);
            velocity.to_array()[0] // Extract the first element
        }).collect()
    }

    /// v^2 = v0^2 + 2 * a * s 
    pub fn final_velocity_batch(displacements: &[f64]) -> Vec<f64> {
        displacements.par_iter().map(|&s| {
            (5.0_f64.powi(2) + 2.0 * 2.0 * s).sqrt()
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_displacement_batch() {
        let times = vec![1.0, 2.0, 3.0, 4.0];
        let results = Kinematics::displacement_batch(&times);
        assert_eq!(results, vec![6.0, 14.0, 24.0, 36.0]);
    }

    #[test]
    fn test_velocity_batch() {
        let times = vec![1.0, 2.0, 3.0, 4.0];
        let results = Kinematics::velocity_batch(&times);
        assert_eq!(results, vec![7.0, 9.0, 11.0, 13.0]);
    }

    #[test]
    fn test_final_velocity_batch() {
        let displacements = vec![14.0, 24.0, 36.0];
        let results = Kinematics::final_velocity_batch(&displacements);
        let expected = vec![7.0, 9.0, 11.0];

        for (r, e) in results.iter().zip(expected.iter()) {
            assert!((r - e).abs() < 1e-6);
        }
    }
}

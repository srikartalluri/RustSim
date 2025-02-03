use rayon::prelude::*;
use std::simd::{Simd, f64x4};

const G: f64 = 6.67430e-11; // Gravitational constant

#[derive(Debug, Clone, Copy)]
pub struct Body {
    pub position: [f64; 3],
    pub velocity: [f64; 3],
    pub mass: f64,
}

impl Body {
    pub fn new(position: [f64; 3], velocity: [f64; 3], mass: f64) -> Self {
        Self { position, velocity, mass }
    }
}

/// Computes gravitational force between two bodies
fn compute_force(a: &Body, b: &Body) -> [f64; 3] {
    let mut force = [0.0; 3];
    let mut distance_squared = 0.0;
    
    // Compute distance vector and squared distance
    let mut r = [0.0; 3];
    for i in 0..3 {
        r[i] = b.position[i] - a.position[i];
        distance_squared += r[i] * r[i];
    }

    let distance = distance_squared.sqrt() + 1e-9; // Avoid division by zero
    let magnitude = G * a.mass * b.mass / (distance * distance);

    for i in 0..3 {
        force[i] = magnitude * (r[i] / distance);
    }
    force
}

/// Updates positions and velocities of bodies in parallel
pub fn update_bodies(bodies: &mut Vec<Body>, dt: f64) {
    let forces: Vec<[f64; 3]> = bodies
        .par_iter()
        .map(|a| {
            let mut total_force = [0.0; 3];
            for b in bodies.iter() {
                if a as *const _ != b as *const _ {
                    let f = compute_force(a, b);
                    for i in 0..3 {
                        total_force[i] += f[i];
                    }
                }
            }
            total_force
        })
        .collect();

    // Update positions and velocities using Newton’s laws
    bodies.par_iter_mut().enumerate().for_each(|(i, body)| {
        for j in 0..3 {
            let acceleration = forces[i][j] / body.mass;
            body.velocity[j] += acceleration * dt;
            body.position[j] += body.velocity[j] * dt;
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_force() {
        let body1 = Body::new([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 5.0);
        let body2 = Body::new([1.0, 0.0, 0.0], [0.0, 0.0, 0.0], 10.0);
        let force = compute_force(&body1, &body2);

        assert!(force[0] > 0.0); // Should attract
    }

    #[test]
    fn test_update_bodies() {
        let mut bodies = vec![
            Body::new([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 5.0),
            Body::new([1.0, 0.0, 0.0], [0.0, 0.0, 0.0], 10.0),
        ];
        update_bodies(&mut bodies, 0.01);
        
        assert!(bodies[0].velocity[0] < 0.0); // Should move toward the larger mass
        assert!(bodies[1].velocity[0] > 0.0);
    }
}

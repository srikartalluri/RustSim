use rayon::prelude::*;
use std::simd::{Simd, f64x4};
use nalgebra::{Vector3, Matrix3, Quaternion, UnitQuaternion};

#[derive(Debug, Clone, Copy)]
pub struct RigidBody {
    pub position: Vector3<f64>,
    pub velocity: Vector3<f64>,
    pub angular_velocity: Vector3<f64>,
    pub orientation: UnitQuaternion<f64>,
    pub mass: f64,
    pub inverse_inertia: Matrix3<f64>,
}

impl RigidBody {
    /// Creates a new rigid body with given parameters
    pub fn new(position: [f64; 3], velocity: [f64; 3], mass: f64, inertia_tensor: [[f64; 3]; 3]) -> Self {
        let inertia_matrix = Matrix3::from(inertia_tensor);
        Self {
            position: Vector3::from_row_slice(&position),
            velocity: Vector3::from_row_slice(&velocity),
            angular_velocity: Vector3::zeros(),
            orientation: UnitQuaternion::identity(),
            mass,
            inverse_inertia: inertia_matrix.try_inverse().unwrap_or(Matrix3::identity()), // Precompute inverse inertia tensor
        }
    }

    /// Applies a force to the rigid body
    pub fn apply_force(&mut self, force: [f64; 3], dt: f64) {
        let force_vector = Vector3::from_row_slice(&force);
        self.velocity += force_vector * (dt / self.mass);
    }

    /// Applies a torque to the rigid body
    pub fn apply_torque(&mut self, torque: [f64; 3], dt: f64) {
        let torque_vector = Vector3::from_row_slice(&torque);
        let angular_acceleration = self.inverse_inertia * torque_vector;
        self.angular_velocity += angular_acceleration * dt;
    }

    /// Updates position and orientation based on velocity and angular velocity
    pub fn update(&mut self, dt: f64) {
        // Update linear motion
        self.position += self.velocity * dt;

        // Update angular motion using quaternions
        let angular_velocity_quat = Quaternion::new(0.0, self.angular_velocity.x, self.angular_velocity.y, self.angular_velocity.z);
        let delta_orientation = 0.5 * angular_velocity_quat * self.orientation.quaternion();
        self.orientation = UnitQuaternion::from_quaternion(self.orientation.quaternion() + delta_orientation * dt);
    }
}

/// Updates all rigid bodies in parallel
pub fn update_rigid_bodies(bodies: &mut Vec<RigidBody>, dt: f64) {
    bodies.par_iter_mut().for_each(|body| {
        body.update(dt);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rigid_body_motion() {
        let mut body = RigidBody::new(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            10.0,
            [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        );

        body.apply_force([10.0, 0.0, 0.0], 1.0);
        body.update(1.0);
        
        assert!(body.position.x > 1.0); // Object should move forward
    }

    #[test]
    fn test_rigid_body_rotation() {
        let mut body = RigidBody::new(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            10.0,
            [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        );

        body.apply_torque([0.0, 0.0, 1.0], 1.0);
        body.update(1.0);

        assert!(body.angular_velocity.z > 0.0); // Should have angular motion
    }
}

//! # Physics Engine
//! This library provides high-performance physics simulations using Rust and parallel computing.

pub mod kinematics;
pub mod nbody;
pub mod fluids;
pub mod rigid_body;

use pyo3::prelude::*;

/// Python bindings for the physics engine
#[pymodule]
fn physics_engine(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<KinematicsPy>()?;
    m.add_class::<RigidBodyPy>()?;
    Ok(())
}

// === PYTHON WRAPPER IMPLEMENTATION === //
use kinematics::Kinematics;
use rigid_body::RigidBody;

/// Python wrapper for Kinematics
#[pyclass]
struct KinematicsPy {
    inner: Kinematics,
}

#[pymethods]
impl KinematicsPy {
    #[new]
    fn new(initial_position: f64, initial_velocity: f64, acceleration: f64) -> Self {
        Self { inner: Kinematics::new(initial_position, initial_velocity, acceleration) }
    }

    fn displacement(&self, time: f64) -> f64 {
        self.inner.displacement(time)
    }

    fn velocity(&self, time: f64) -> f64 {
        self.inner.velocity(time)
    }
}

/// Python wrapper for RigidBody
#[pyclass]
struct RigidBodyPy {
    inner: RigidBody,
}

#[pymethods]
impl RigidBodyPy {
    #[new]
    fn new(position: [f64; 3], velocity: [f64; 3], mass: f64, inertia: [[f64; 3]; 3]) -> Self {
        Self { inner: RigidBody::new(position, velocity, mass, inertia) }
    }

    fn apply_force(&mut self, force: [f64; 3], dt: f64) {
        self.inner.apply_force(force, dt);
    }

    fn apply_torque(&mut self, torque: [f64; 3], dt: f64) {
        self.inner.apply_torque(torque, dt);
    }

    fn update(&mut self, dt: f64) {
        self.inner.update(dt);
    }
}

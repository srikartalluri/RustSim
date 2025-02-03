# RustSim

RustSim is a high-speed, parallelized physics engine written in Rust, with a Python wrapper for easy usage. It provides optimized implementations for kinematics, rigid body dynamics, fluid dynamics, and n-body simulations using SIMD, Rayon, and PyO3.

```
physolve/
│── physics_engine/
│   ├── src/
│   │   ├── lib.rs         
│   │   ├── kinematics.rs  # Kinematics module
│   │   ├── rigid_body.rs  # Rigid body physics
│   │   ├── nbody.rs       # N-body simulation
│   │   ├── fluids.rs      # Fluid dynamics
│   ├── Cargo.toml         
│── wrapper/               # Python wrapper top level folder
│   ├── physics.py         
│   ├── __init__.py        
│   ├── test_physics.py    # Unit Python tests
│── examples/              # Examples for python use

```

The setup is pretty simple:

First ```pip install maturin numpy```

Then ```maturin develop``` to build the Rust library and install the Python wrapper.

I timed the speedup using things like 

```python -m timeit -s "from wrapper.physics import Kinematics" "Kinematics(0, 10, -9.8).displacement(2)"```


import physics_engine

class Kinematics:
    """ Python wrapper for Rust-based Kinematics """
    def __init__(self, initial_position, initial_velocity, acceleration):
        self.obj = physics_engine.KinematicsPy(initial_position, initial_velocity, acceleration)

    def displacement(self, time):
        return self.obj.displacement(time)

    def velocity(self, time):
        return self.obj.velocity(time)


class RigidBody:
    """ Python wrapper for Rust-based Rigid Body Dynamics """
    def __init__(self, position, velocity, mass, inertia):
        self.obj = physics_engine.RigidBodyPy(position, velocity, mass, inertia)

    def apply_force(self, force, dt):
        self.obj.apply_force(force, dt)

    def apply_torque(self, torque, dt):
        self.obj.apply_torque(torque, dt)

    def update(self, dt):
        self.obj.update(dt)

    def get_position(self):
        return self.obj.position

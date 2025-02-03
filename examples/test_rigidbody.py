from wrapper.physics import RigidBody

body = RigidBody([0, 0, 0], [1, 0, 0], 10, [[1, 0, 0], [0, 1, 0], [0, 0, 1]])
body.apply_force([10, 0, 0], 1.0)
body.update(1.0)
print("New Position:", body.get_position())

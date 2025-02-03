from wrapper.physics import Kinematics

kin = Kinematics(0, 10, -9.8)
print(f"Displacement after 2s: {kin.displacement(2)}")
print(f"Velocity after 2s: {kin.velocity(2)}")

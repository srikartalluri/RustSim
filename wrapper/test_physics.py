import unittest
from physics import Kinematics, RigidBody

class TestKinematics(unittest.TestCase):
    def test_displacement(self):
        kin = Kinematics(0, 10, -9.8)
        self.assertAlmostEqual(kin.displacement(2), 0.4, places=1)

    def test_velocity(self):
        kin = Kinematics(0, 10, -9.8)
        self.assertAlmostEqual(kin.velocity(2), -9.6, places=1)


class TestRigidBody(unittest.TestCase):
    def test_apply_force(self):
        body = RigidBody([0, 0, 0], [1, 0, 0], 10, [[1, 0, 0], [0, 1, 0], [0, 0, 1]])
        body.apply_force([10, 0, 0], 1.0)
        body.update(1.0)
        self.assertGreater(body.get_position()[0], 1.0)

if __name__ == '__main__':
    unittest.main()

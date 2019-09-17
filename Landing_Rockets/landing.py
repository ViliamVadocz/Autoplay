import numpy as np
import matplotlib.pyplot as plt

def f(x, a):
    """Modified sigmoid function. Larger values for a give steeper rise."""
    return 2 / (1 + np.exp(a*x)) - 1

def cap(value : float, minimum : float, maximum : float) -> float:
    """Caps the value at given minumum and maximum.
    
    Arguments:
        value {float} -- The value being capped.
        minimum {float} -- Smallest value.
        maximum {float} -- Largest value.
    
    Returns:
        float -- The capped value or the original value if within range.
    """
    if value > maximum:
        return maximum
    elif value < minimum:
        return minimum
    else:
        return value

class Rocket:
    def __init__(self):
        """Initialises the rocket.
        
        Arguments:
            height {float} -- The initial height for the rocket.
        """

        # Properties
        self.gravity = np.array([0.0, -9.81])           # Acceleration due to gravity [m/s^2]
        self.mass : float = 2E5                         # Mass [kg]
        self.max_thrust : float = 5E6                   # Maximum thrust output [N]
        self.max_turn : float = np.pi / 8               # Maximum turn acceleration [radians/s^2]

        # Kinematics
        self.ang : float = np.pi / 2                    # Angle [radians]
        self.ang_vel : float = 0.0                      # Anglular velocity [randian/s]
        self.acc = self.gravity                         # Acceleration vector [m/s^2]
        self.vel = np.random.uniform(-10, 10, (2,))     # Velocity vector [m/s]
        self.pos = np.array([0.0, 1000.0])              # Position vector [m]

        # Tracking
        #self.path : list = [self.pos[:,np.newaxis]]     # List of past positions of the rocket
        self.thrust_used : float = 0.0                  # Collective thrust used [N]
        

    def simulate(self, throttle, turn, dt):
        """Simulates the rocket.
        
        Arguments:
            throttle {float} -- The input throttle.
            turn {float} -- The input turn.
        """
        # Turns the rocket.
        self.ang_vel += turn * self.max_turn * dt
        self.ang += self.ang_vel * dt

        # Calculates propulsion vector from direction.
        prop_vec = np.array([np.cos(self.ang),np.sin(self.ang)])

        # Calculates acceleration, velocity, and position.
        throttle_acc = throttle * self.max_thrust * prop_vec / self.mass
        self.acc = (throttle_acc + np.array([0.0, -9.81])) * dt
        self.vel += self.acc
        self.pos += self.vel * dt

        # Tracking
        #self.path += [self.pos[:,np.newaxis]]
        self.thrust_used += throttle # + self.turn


class Landing:
    def __init__(self):
        """Initialises the landing sequence."""
        # Creates the rocket.
        self.rocket = Rocket()

        # Initialises the success of the landing.
        self.success = None

    def loop(self, seconds: int, tps : int = 120):
        """Simulate the rocket for some amount of seconds,
        at the end of which if the rocket has landed, assigns self.success to True.
        
        Arguments:
            seconds {int} -- Amount of seconds to simulate for.
        
        Keyword Arguments:
            tps {int} -- How many simulation ticks to run per second. (default: {120})
        """
        dt = 1.0 / tps
        path = np.zeros((seconds * tps, 2))
        for i in range(seconds * tps):
            turn, throttle = get_controls(self.rocket)
            turn = cap(turn, -1.0, 1.0)
            throttle = cap(throttle, 0.0, 1.0)

            # Simulate the motion of the rocket.
            self.rocket.simulate(throttle, turn, dt)
            #print(self.rocket.pos)
            path[i] += self.rocket.pos

            if self.rocket.pos[1] <= 0.0:
                break

        # Tests if the rocket has landed safely.
        LANDING_PAD_SIZE = 25
        MAXIMUM_LAND_VEL = 10
        if self.rocket.pos[1] <= 0.0 and np.linalg.norm(self.rocket.vel) <= MAXIMUM_LAND_VEL and abs(self.rocket.pos[0]) <= LANDING_PAD_SIZE:

            self.success = True
        else:
            self.success = False

        path = path[~np.all(path == 0, axis=1)]
        return path

def get_controls(rocket):
    """Gets the turn and throttle input for this tick.
    
    Arguments:
        rocket {Rocket} -- The rocket information.

    Returns:
        float -- Turn input from -1.0 to 1.0. Positive is counterclockwise.
        float -- Throttle input from 0.0 to 1.0
    """
    # Temporary test script to manually control the rocket descent. 
    # Will be controlled by NN later.
    opposite_vel_angle = np.arctan2(rocket.vel[1],rocket.vel[0]) + np.pi
    angle_diff = opposite_vel_angle - rocket.ang
    turn = f(angle_diff, 5)
    
    if abs(rocket.angle - np.pi / 2) > 0.3:
        throttle = 1.0 
    elif True:
        # TODO Suicide burn.
        throttle = 1.0
    else:
        throttle = 0.0
    
    return turn, throttle



test = Landing()
path = test.loop(100)

a = path[:,0]
b = path[:,1]

plt.plot(a,b,'-b')
plt.show()
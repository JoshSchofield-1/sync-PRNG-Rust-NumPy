import random as rnd
import numpy as np

# Create a json file containing a list of 10000 random f64 samples between 0-1 from Numpy Mersenne Twister PRNG
# We cant use the seed directly for numpy so we use the state from python random.random
# This allows us to sync random number generation across Python and Rust

# seed must be chosen from valid_seeds.txt 
# (this can be extended to include more seeds by increasein the for loop in valid_seeds.py)
seedval = 827680
rnd.seed(seedval)
state = rnd.getstate()
new_state = ('MT19937', [int(s) for s in list(state[1]) ], 624, 0, 0.0)
np.random.set_state(new_state)

f = open("../data/rng_vals_seed"+str(seedval)+".json", 'w+')

f.write("[")
for i in range(10000):
    if i != 0:
        f.write(", ")
    f.write(str(np.random.random()))
f.write("]")
f.close()

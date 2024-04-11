import random as rnd

# Create a file of seeds where the state of the PRNG only contains positive integers

f = open("../data/valid_seed.txt", 'w')
f.write("[")
for i in range(10):
    seed = rnd.randint(1000, 999999)
    rnd.seed(seed)
    if i != 0:
        f.write(", ")
    good = True
    for val in rnd.getstate()[1]:
        if val < 0:
            good = false
            break
    if good == True:
        # print(seed)
        f.write(str(seed))
f.write("]")
f.close()
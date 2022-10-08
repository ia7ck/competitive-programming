import random
import math

n = random.randint(1, 400)
if random.randint(0, 1):
    m = random.randint(1, int(math.sqrt(n)) + 1)
else:
    m = random.randint(1, n)

print(n, m)

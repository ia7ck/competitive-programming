import random

h = random.randint(1, 8)
w = random.randint(1, 8)
n = random.randint(0, h * w)
ab = set([(random.randint(1, h), random.randint(1, w)) for _ in range(n)])
n = len(ab)
print(h, w, n)
for (a, b) in ab:
    print(a, b)

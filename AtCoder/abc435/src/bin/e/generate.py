import random

n = random.randint(1, 10)
q = random.randint(1, 10)
print(n, q)
for _ in range(q):
    L = random.randint(1, n)
    R = random.randint(1, n)
    print(min(L, R), max(L, R))

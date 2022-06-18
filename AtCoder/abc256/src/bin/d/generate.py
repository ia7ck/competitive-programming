import random

n = random.randint(1, 4)
print(n)
for _ in range(n):
    l = random.randint(1, 99)
    r = random.randint(l + 1, 100)
    print(l, r)

import random

n = random.randint(1, 10)
m = random.randint(0, n)
a = random.sample(range(1, n + 1), m)
print(n, m)
print(*sorted(a))

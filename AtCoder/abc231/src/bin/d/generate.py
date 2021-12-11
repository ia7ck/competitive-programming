import random

n = random.randint(2, 7)
m = random.randint(0, n * 2)
ab = set()
for _ in range(m):
    a = random.randint(1, n - 1)
    b = random.randint(a + 1, n)
    ab.add((a, b))
print(n, len(ab))
for (a, b) in ab:
    print(a, b)

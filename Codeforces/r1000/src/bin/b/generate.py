import random

n = random.randint(1, 10)
l = random.randint(1, n)
r = random.randint(l, n)
a = [random.randint(1, 10) for _ in range(n)]

print(1)
print(n, l, r)
print(*a)

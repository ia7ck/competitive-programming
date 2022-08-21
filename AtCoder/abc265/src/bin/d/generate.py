import random

n = random.randint(4, 20)
p = random.randint(1, 10)
q = random.randint(1, 10)
r = random.randint(1, 10)
a = [random.randint(1, 10) for _ in range(n)]
print(n, p, q, r)
print(*a)

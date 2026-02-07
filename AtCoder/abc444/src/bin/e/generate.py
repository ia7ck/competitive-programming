import random

n = random.randint(2, 10)
d = random.randint(1, 10)
a = [random.randint(1, 10) for _ in range(n)]

print(n, d)
print(*a)

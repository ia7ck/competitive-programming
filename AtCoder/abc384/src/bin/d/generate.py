import random

n = random.randint(1, 10)
s = random.randint(1, 1000)
a = [random.randint(1, 100) for _ in range(n)]

print(n, s)
print(*a)

import random

n = random.randint(1, 15)
a = [random.randint(1, n) for _ in range(n)]

print(n)
print(*a)

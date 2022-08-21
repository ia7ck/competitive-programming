import random

n = random.randint(3, 10)
a = [random.randint(1, 1_000_000 - 1) for _ in range(n)]

print(n)
print(*a)

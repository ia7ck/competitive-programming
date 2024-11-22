import random

n = random.randint(1, 10)
a = [random.randint(1, n) for _ in range(n)]

print(n)
print(*a)

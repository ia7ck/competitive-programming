import random

n = random.randint(1, 10)
print(n)
a = [random.randint(0, 1) for _ in range(n)]
print(*a)

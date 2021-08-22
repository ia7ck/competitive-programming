import random

n = random.randint(2, 6)
print(n)
a = [random.randint(0, 1 << 10) for _ in range(n)]
print(*a)

import random

n = random.randint(1, 6)
print(n)
a = [random.randint(1, 10) for _ in range(n)]
print(*a)

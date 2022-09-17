import random

n = random.randint(2, 10)
a = [random.randint(0, 100) for _ in range(n)]

print(n)
print(*a)

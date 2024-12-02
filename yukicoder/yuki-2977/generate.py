import random

n = random.randint(2, 10)
k = random.randint(1, n * (n - 1) // 2)
a = [random.randint(1, 100) for i in range(n)]

print(n, k)
print(*a)

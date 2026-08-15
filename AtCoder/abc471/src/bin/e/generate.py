import random

n = random.randint(1, 10)
k = random.randint(1, n)
a = [random.randint(1, 10) for i in range(n)]

print(n, k)
print(*a)

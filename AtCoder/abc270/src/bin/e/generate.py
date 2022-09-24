import random

n = random.randint(1, 10)
a = [random.randint(0, 10) for _ in range(n)]
k = random.randint(1, sum(a))

print(n, k)
print(*a)

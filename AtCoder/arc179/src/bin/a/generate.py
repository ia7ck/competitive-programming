import random

n = random.randint(1, 5)
k = random.randint(-10, 10)
a = [random.randint(-10, 10) for i in range(n)]
print(n, k)
print(*a)

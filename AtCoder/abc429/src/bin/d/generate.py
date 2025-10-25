import random

n = random.randint(1, 10)
m = random.randint(1, 10)
c = random.randint(1, n)
a = [random.randint(0, m - 1) for i in range(n)]

print(n, m, c)
print(*a)

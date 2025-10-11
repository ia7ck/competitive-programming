import random

n = random.randint(1, 5)
m = random.randint(1, 10)
a = [random.randint(0, m - 1) for i in range(n)]

print(n, m)
print(*a)

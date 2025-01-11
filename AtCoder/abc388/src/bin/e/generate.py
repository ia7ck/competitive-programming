import random

n = random.randint(2, 8)
a = [random.randint(1, n) for i in range(n)]
a.sort()

print(n)
print(*a)

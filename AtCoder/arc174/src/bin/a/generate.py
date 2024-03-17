import random

n = random.randint(1, 3)
c = random.randint(-10, 10)
a = [random.randint(-10, 10) for i in range(n)]

print(n, c)
print(*a)

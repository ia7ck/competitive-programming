import random

n = random.randint(1, 4)
m = random.randint(1, 4)
a = [random.randint(1, 9) for _ in range(n)]
b = [random.randint(1, 9) for _ in range(m)]

print(n, m)
print(*a)
print(*b)

import random

n = random.randint(1, 10)
p = [random.randint(1, i + 1) for i in range(n)]

print(n)
print(*p)

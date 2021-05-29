import random

n = random.randint(3, 10)
print(n)
for _ in range(n):
    x = random.randint(-10, 10)
    y = random.randint(-10, 10)
    print(x, y)

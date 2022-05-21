import random

n = random.randint(1, 5)
l = random.randint(n, 10)
a = [1] * n
for _ in range(random.randint(0, l - n)):
    i = random.randint(0, n - 1)
    a[i] += 1
print(n, l)
print(*a)


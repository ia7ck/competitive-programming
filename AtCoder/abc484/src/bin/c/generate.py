import random

n = random.randint(1, 10)
a = [random.randint(1, n) for i in range(n)]
if random.randint(0, 1):
    # Yes
    i = random.randint(0, n - 1)
    j = random.randint(0, n - 1)
    y = a[i]
    b = a[:i] + a[(i + 1) :]
    b.insert(j, y)
else:
    a = [random.randint(1, n) for i in range(n)]
    b = [random.randint(1, n) for i in range(n)]
print(n)
print(*a)
print(*b)

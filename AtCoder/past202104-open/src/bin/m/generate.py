import random

n = 11
a = [random.randint(1, 9) for _ in range(n)]
print(n)
print(*a)
q = 10
print(q)
for _ in range(q):
    l = random.randint(1, n - 1)
    r = random.randint(l + 1, n)
    x = random.randint(1, 9)
    print(l, r, x)

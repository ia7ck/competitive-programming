import random

n = random.randint(1, 10)
a = [random.randint(1, 100) for _ in range(n)]
print(n)
print(*a)
q = random.randint(1, 3)
print(q)
for _ in range(q):
    l = random.randint(1, n)
    r = random.randint(l, n)
    k = random.randint(1, 10)
    print(l, r, k)

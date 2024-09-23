import random

t = 1
n = random.randint(1, 10)
q = random.randint(1, 10)
a = [random.randint(1, 10) for i in range(n)]
print(t)
print(n, q)
print(*a)
for i in range(q):
    l = random.randint(1, n)
    r = random.randint(l, n)
    print(l, r)

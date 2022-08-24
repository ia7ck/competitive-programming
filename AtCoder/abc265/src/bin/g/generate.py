import random

n = random.randint(1, 10)
q = random.randint(1, 10)
a = [random.randint(0, 2) for _ in range(n)]
print(n, q)
print(*a)
for _ in range(q):
    op = random.randint(1, 2)
    l = random.randint(1, n)
    r = random.randint(l, n)
    if op == 1:
        print(op, l, r)
    else:
        s = random.randint(0, 2)
        t = random.randint(0, 2)
        u = random.randint(0, 2)
        print(op, l, r, s, t, u)

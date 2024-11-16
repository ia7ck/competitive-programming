import random

n = random.randint(1, 10)
q = random.randint(1, 10)
print(n, q)
for i in range(q):
    op = random.randint(1, 2)
    if op == 1:
        x = random.randint(1, n)
        c = random.randint(1, n)
        print(op, x, c)
    else:
        c = random.randint(1, n)
        print(op, c)

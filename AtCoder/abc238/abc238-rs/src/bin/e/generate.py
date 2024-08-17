import random

n = random.randint(1, 10)
q = random.randint(1, min(10, n * (n + 1) / 2))
print(n, q)

lr = set()
while len(lr) < q:
    l = random.randint(1, n)
    r = random.randint(l, n)
    lr.add((l, r))

for l, r in lr:
    print(l, r)
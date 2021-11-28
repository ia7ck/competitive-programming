import random

h = random.randint(2, 10)
w = random.randint(2, 10)
c = 9
q = random.randint(1, 20)
print(h, w, c, q)
for _ in range(q):
    t = random.randint(1, 2)
    if t == 1:
        n = random.randint(1, h)
        c = random.randint(1, c)
    else:
        n = random.randint(1, w)
        c = random.randint(1, c)
    print(t, n, c)


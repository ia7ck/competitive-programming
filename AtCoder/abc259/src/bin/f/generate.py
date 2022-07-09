import random

n = random.randint(2, 10)
edges = []
d = [0] * n
for i in range(1, n):
    p = random.randint(0, i - 1)
    d[p] += 1
    d[i] += 1
    edges.append((p + 1, i + 1))

print(n)
lim = [random.randint(0, x) for x in d]
print(*lim)
for a, b in edges:
    c = random.randint(-10, 10)
    print(a, b, c)

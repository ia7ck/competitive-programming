import random

n = random.randint(2, 10)
edges = []
for i in range(1, n):
    p = random.randint(0, i - 1)
    edges.append((p, i))

perm = [i for i in range(n)]
random.shuffle(perm)
edges = [(perm[u], perm[v]) for u, v in edges]

print(1)
print(n)
for u, v in edges:
    print(u + 1, v + 1)

import random

n = random.randint(1, 10)
a = [random.randint(1, n) for i in range(n)]
print(n)
print(*a)
p = [0] * n
for i in range(1, n):
    p[i] = random.randint(0, i - 1)
order = [i for i in range(n)]
random.shuffle(order)
for i in range(1, n):
    u, v = p[i], i
    print(order[u] + 1, order[v] + 1)
# ランダムになってる？

n = int(input())
lim = list(map(int, input().split()))
edges = []
for _ in range(n - 1):
    a, b, c = map(int, input().split())
    edges.append((a - 1, b - 1, c))

answer = -(10 ** 18)
for bits in range(1 << (n - 1)):
    d = [0] * n
    cost = 0
    for i in range(n - 1):
        if bits >> i & 1:
            a, b, c = edges[i]
            d[a] += 1
            d[b] += 1
            cost += c
    ok = True
    for v in range(n):
        if d[v] > lim[v]:
            ok = False
    if ok:
        if answer < cost:
            answer = cost
print(answer)

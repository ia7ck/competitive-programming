n, k = map(int, input().split())
a = [list(map(int, input().split())) for _ in range(n)]
g = [[0] * (n * k) for _ in range(n * k)]
for i in range(n):
    for j in range(n):
        for p in range(k):
            for q in range(k):
                g[i + n * p][j + n * q] = a[i][j]
INF = 1000000000
d = [[INF] * (n * k) for _ in range(n * k)]
for v in range(n * k):
    d[v][v] = 0
for s in range(n * k):
    for t in range(n * k):
        if g[s][t] == 1:
            d[s][t] = 1
for v in range(n * k):
    for s in range(n * k):
        for t in range(n * k):
            d[s][t] = min(d[s][t], d[s][v] + d[v][t])
q = int(input())
for _ in range(q):
    s, t = map(int, input().split())
    s -= 1
    t -= 1
    if d[s][t] == INF:
        print(-1)
    else:
        print(d[s][t])
    

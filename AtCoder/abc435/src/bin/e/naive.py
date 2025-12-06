n, q = map(int, input().split())
queries = [list(map(int, input().split())) for _ in range(q)]

assert n <= 100

black = [False] * n
for query in queries:
    L, R = query
    for i in range(L - 1, R):
        black[i] = True
    ans = 0
    for i in range(n):
        if not black[i]:
            ans += 1
    print(ans)

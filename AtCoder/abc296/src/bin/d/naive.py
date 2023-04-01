n, m = map(int, input().split())

inf = 10 ** 18
ans = inf
for a in range(1, n + 1):
    for b in range(1, n + 1):
        if a * b >= m:
            if ans > a * b:
                ans = a * b
if ans == inf:
    print(-1)
else:
    print(ans)

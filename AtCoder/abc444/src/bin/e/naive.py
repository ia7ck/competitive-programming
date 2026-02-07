n, d = map(int, input().split())
a = list(map(int, input().split()))

ans = 0
for L in range(n):
    for R in range(L, n):
        ok = True
        for i in range(L, R + 1):
            for j in range(i + 1, R + 1):
                if abs(a[i] - a[j]) < d:
                    ok = False
        if ok:
            ans += 1
print(ans)

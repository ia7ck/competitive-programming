n, c = map(int, input().split())
a = list(map(int, input().split()))

ans = sum(a)
for i in range(n):
    for j in range(i, n):
        b = a.copy()
        for k in range(i, j + 1):
            b[k] *= c
        ans = max(ans, sum(b))
print(ans)

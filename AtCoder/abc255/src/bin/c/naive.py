x, a, d, n = map(int, input().split())

seq = [a + d * i for i in range(n)]
ans = 1_000_000_000
for s in seq:
    d = abs(s - x)
    if ans > d:
        ans = d
print(ans)

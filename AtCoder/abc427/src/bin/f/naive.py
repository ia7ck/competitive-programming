n, m = map(int, input().split())
a = list(map(int, input().split()))

ans = 0
for bits in range(1 << n):
    idx = [i for i in range(n) if bits >> i & 1 == 1]
    ok = True
    for i in range(len(idx)):
        if i + 1 < len(idx) and idx[i] + 1 == idx[i + 1]:
            ok = False
    s = sum([a[i] for i in idx])
    if ok and s % m == 0:
        ans += 1
print(ans)

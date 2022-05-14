n = int(input())
a = list(map(int, input().split()))

ans = 1_000_000_000
for bits in range(1 << n):
    cost = 0
    count = [0] * n
    for i in range(n):
        if bits >> i & 1:
            cost += a[i]
            count[i] += 1
            count[(i + 1) % n] += 1
    ok = True
    for c in count:
        if c == 0:
            ok = False
            break
    if ok:
        if cost < ans:
            ans = cost
print(ans)


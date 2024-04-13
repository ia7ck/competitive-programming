import math

n, m = map(int, input().split())
a = list(map(int, input().split()))

ans = 0
for bits in range(1, 1 << n):
    l = 1
    for i in range(n):
        if  bits >> i & 1 == 1:
            l = math.lcm(l, a[i])
    if l == m:
        ans += 1
print(ans % 998244353)

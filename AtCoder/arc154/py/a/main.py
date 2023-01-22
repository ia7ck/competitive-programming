mod = 998244353

n = int(input())
a = list(input())
b = list(input())

for i in range(n):
    if a[i] > b[i]:
        a[i], b[i] = b[i], a[i]

a = int("".join(a))

prod = [0] * 10
for d in range(1, 10):
    prod[d] = a * d % mod

ans = 0
for i, d in enumerate(reversed(b)):
    d = int(d)
    ans += prod[d] * pow(10, i, mod)
    ans %= mod
print(ans)

n, m, c = map(int, input().split())
a = list(map(int, input().split()))

assert m <= 100
f = [0] * m
for x in a:
    f[x] += 1

ans = 0
for i in range(m):
    j = i + 1
    x = 0
    while x < c:
        x += f[j % m]
        j += 1
    ans += x

print(ans)

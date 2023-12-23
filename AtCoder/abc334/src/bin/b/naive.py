a, m, l, r = map(int, input().split())
ans = 0
for k in range(-1000, 1000):
    if l <= a + k * m <= r:
        ans += 1
print(ans)

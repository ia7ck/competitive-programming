a, x, m = map(int, input().split())

ans = 0
for i in range(x):
    ans += pow(a, i, m)
    ans %= m
print(ans)

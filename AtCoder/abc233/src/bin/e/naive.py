x = int(input())
ans = 0
for k in range(1000000000):
    a = x // pow(10, k)
    if a == 0:
        break
    ans += a
print(ans)

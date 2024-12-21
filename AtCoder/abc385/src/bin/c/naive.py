n = int(input())
h = list(map(int, input().split()))

ans = 1
for i in range(n):
    for w in range(1, n):
        x = -1
        L = 0
        for j in range(i, n, w):
            if x == h[j]:
                L += 1
            else:
                ans = max(ans, L)
                x = h[j]
                L = 1
        ans = max(ans, L)
print(ans)

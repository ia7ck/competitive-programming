n = int(input())
a = list(map(int, input().split()))

ans = 0
for i in range(n):
    for j in range(n):
        if i == j:
            continue
        for k in range(n):
            if i == k or j == k:
                continue
            y = int(f"{a[i]}{a[j]}{a[k]}")
            if ans < y:
                ans = y
print(ans)

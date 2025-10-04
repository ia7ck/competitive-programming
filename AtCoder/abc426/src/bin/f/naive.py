n = int(input())
a = list(map(int, input().split()))
q = int(input())
for _ in range(q):
    l, r, k = map(int, input().split())
    ans = 0
    for i in range(l - 1, r):
        if a[i] < k:
            ans += a[i]
        else:
            ans += k
        a[i] = max(a[i] - k, 0)
    print(ans)

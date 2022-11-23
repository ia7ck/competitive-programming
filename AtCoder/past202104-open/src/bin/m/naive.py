n = int(input())
a = list(map(int, input().split()))
q = int(input())
for _ in range(q):
    l, r, x = map(int, input().split())
    for i in range(l - 1, r):
        a[i] = x
    ans = 0
    for i in range(n):
        for j in range(i + 1, n):
            if a[i] == a[j]:
                ans += 1
    print(ans)

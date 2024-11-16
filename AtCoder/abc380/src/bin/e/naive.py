n, q = map(int, input().split())
a = [i for i in range(n)]
for _q in range(q):
    op, *rest = map(int, input().split())
    if op == 1:
        x, c = rest
        x -= 1
        c -= 1
        left = right = x
        for i in reversed(range(0, x)):
            if a[i] == a[x]:
                left = i
            else:
                break
        for i in range(x + 1, n):
            if a[i] == a[x]:
                right = i
            else:
                break
        for i in range(left, right + 1):
            a[i] = c
    else:
        (c,) = rest
        c -= 1
        ans = 0
        for i in range(n):
            if a[i] == c:
                ans += 1
        print(ans)

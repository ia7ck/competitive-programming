def solve(n, l, r, a):
    ans = sum(a[l - 1 : r])
    for bits in range(1 << n):
        b = []
        for i in range(n):
            if bits >> i & 1:
                b.append(a[i])
        new_a = a[:]
        for i in range(n):
            if bits >> i & 1:
                new_a[i] = b.pop()
        s = sum(new_a[l - 1 : r])
        ans = min(ans, s)
    print(ans)


t = int(input())
for i in range(t):
    n, l, r = map(int, input().split())
    a = list(map(int, input().split()))
    solve(n, l, r, a)

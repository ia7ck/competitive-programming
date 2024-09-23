def solve(n, q, a, queries):
    for [l, r] in queries:
        s = set()
        for i in range(l - 1, r):
            if a[i] in s:
                s.remove(a[i])
            else:
                s.add(a[i])
        if len(s) == 0:
            print("YES")
        else:
            print("NO")

t = int(input())
for _t in range(t):
    n, q = map(int, input().split())
    a = list(map(int, input().split()))
    queries = [list(map(int, input().split())) for i in range(q)]
    solve(n, q, a, queries)


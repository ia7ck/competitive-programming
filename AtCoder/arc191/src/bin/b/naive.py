def solve(n, k):
    xs = []
    for x in range(1, 1000):
        if x ^ n == x % n:
            xs.append(x)
    if len(xs) < k:
        print(-1)
    else:
        print(xs[k - 1])

t = int(input())

for i in range(t):
    n, k = map(int, input().split())
    solve(n ,k)

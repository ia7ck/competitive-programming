t = int(input())

for _ in range(t):
    n, m = map(int, input().split())

    ans = 0
    for x in range(1, n + 1):
        for y in range(1, n + 1):
            z = int(f"{x}{y}")
            if z % m == (x + y) % m:
                ans += 1

    print(ans)

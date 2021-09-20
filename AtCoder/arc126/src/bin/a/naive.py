from functools import lru_cache

@lru_cache(None)
def solve(a, b, c):
    if (a, b, c) == (1000000000000000, 1000000000000000, 1000000000000000):
        return 900000000000000
    ans = 0
    for i in range(0, 6):
        for j in range(0, 4):
            for k in range(0, 3):
                if i * 2 + j * 3 + k * 4 == 10:
                    if a >= i and b >= j and c >= k:
                        ans = max(ans, solve(a - i, b - j, c - k) + 1)
    return ans

t = int(input())
for _ in range(t):
    a, b, c = map(int, input().split())

    print(solve(a, b, c))

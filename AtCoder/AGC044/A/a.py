#! /usr/bin/env python3

import sys
sys.setrecursionlimit(10 ** 6)

tt = int(input())
for _ in range(tt):
    n, a, b, c, d = map(int, input().split())
    memo = {}
    def solve(k):
        if k == 0:
            return 0
        elif k == 1:
            return d
        elif k in memo:
            return memo[k]
        else:
            res = k * d
            r = k % 2
            for x in [k - r, k - r + 2]:
                if x >= 1 and x // 2 < k:
                    res = min(res, d * abs(k - x) + a + solve(x // 2))
            r = k % 3
            for x in [k - r, k - r + 3]:
                if x >= 1 and x // 3 < k:
                    res = min(res, d * abs(k - x) + b + solve(x // 3))
            r = k % 5
            for x in [k - r, k - r + 5]:
                if x >= 1 and x // 5 < k:
                    res = min(res, d * abs(k - x) + c + solve(x // 5))
            memo[k] = res
            return memo[k]
    print(solve(n))

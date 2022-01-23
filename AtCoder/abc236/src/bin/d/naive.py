n = int(input())
a = []
for _ in range(n * 2 - 1):
    row = list(map(int, input().split()))
    a.append(row)

import itertools
ans = 0
for p in itertools.permutations(range(n * 2)):
    b = 0
    for i in range(n):
        s = min(p[i * 2], p[i * 2 + 1])
        t = max(p[i * 2], p[i * 2 + 1])
        b ^= a[s][t - s - 1]
    if b > ans:
        ans = b
print(ans)
        

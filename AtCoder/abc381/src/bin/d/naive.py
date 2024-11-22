from collections import Counter

n = int(input())
a = list(map(int, input().split()))

ans = 0
for l in range(n):
    for r in range(l, n + 1):
        if (r - l) % 2 == 0:
            b = a[l:r]
            ok = True
            for i in range((r - l) // 2):
                if b[i * 2] != b[i * 2 + 1]:
                    ok = False
                    break
            counter = Counter(b)
            for v in counter.values():
                if v != 2:
                    ok = False
                    break
            if ok:
                ans = max(ans, r - l)
print(ans)

n = int(input())
a = list(map(int, input().split()))

ans = 0
for s in range(0, 1 << n):
    b = [a[i] for i in range(n) if s >> i & 1 == 1]
    if len(b) % 2 == 0 and len(set(b)) * 2 == len(b):
        ok = True
        for i in range(len(b) // 2):
            if b[i * 2] != b[i * 2 + 1]:
                ok = False
                break
        if ok:
            ans = max(ans, len(b))
print(ans)

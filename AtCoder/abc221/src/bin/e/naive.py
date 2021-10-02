n = int(input())
a = list(map(int, input().split()))

ans = 0
for bits in range(1 << n):
    b = [a[i] for i in range(n) if bits >> i & 1]
    if len(b) < 2:
        continue
    if b[0] <= b[-1]:
        ans += 1
print(ans)

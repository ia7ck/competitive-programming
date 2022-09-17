n = int(input())
a = list(map(int, input().split()))

ans = 2
for m in range(2, max(a) + 1):
    s = set()
    for x in a:
        s.add(x % m)
    if ans > len(s):
        ans = len(s)
print(ans)

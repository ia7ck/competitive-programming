l, n1, n2 = map(int, input().split())
a = []
for _ in range(n1):
    v, l1 = map(int, input().split())
    a.extend([v] * l1)
b = []
for _ in range(n2):
    v, l2 = map(int, input().split())
    b.extend([v] * l2)

ans = 0
for i in range(l):
    if a[i] == b[i]:
        ans += 1
print(ans)

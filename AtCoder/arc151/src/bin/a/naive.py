n = int(input())
s = input()
t = input()

ans = "1" * n
found = False
for bits in range(1 << n):
    u = "".join([str(bits >> i & 1) for i in range(n)])
    dist_s = 0
    dist_t = 0
    for i in range(n):
        if s[i] != u[i]:
            dist_s += 1
        if t[i] != u[i]:
            dist_t += 1
    if dist_s == dist_t:
        ans = min(ans, u)
        found = True

if found:
    print(ans)
else:
    print(-1)
    


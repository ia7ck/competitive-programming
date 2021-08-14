s = list(input())
n = len(s)
sub = set()
for bits in range(1, 1 << n):
    ng = False
    for i in range(n - 1):
        if (bits >> i & 1) == 1 and (bits >> (i + 1) & 1) == 1:
            ng = True
    if ng:
        continue
    t = ""
    for i in range(n):
        if bits >> i & 1:
            t += s[i]
    sub.add(t)
print(len(sub))

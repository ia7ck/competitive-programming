n = int(input())
s = input()

ok = set()
for m in range(1, n):
    if n % m != 0:
        continue
    for bits in range(1 << m):
        pat = ['#' if bits >> i & 1 else '.' for i in range(m)]
        t = pat * (n // m)
        yes = True
        for i in range(n):
            if s[i] == '#' or t[i] == '#':
                pass
            else:
                yes = False
                break
        if yes:
            ok.add("".join(t))
import sys
print("ans", len(ok), file=sys.stderr)
print(len(ok))


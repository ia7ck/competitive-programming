import itertools
import sys

def solve(n, a):
    for p in itertools.permutations(a):
        ok = True
        for i in range(n - 1):
            # p[1] / p[0] = p[i+1] / p[i]
            if p[1] * p[i] == p[i+1] * p[0]:
                pass
            else:
                ok = False
                break
        if ok:
            sys.stdout.buffer.write(b"Yes\n")
            return
    sys.stdout.buffer.write(b"No\n")

t = int(input())
for i in range(t):
    n = int(input())
    a = list(map(int, input().split()))
    solve(n, a)
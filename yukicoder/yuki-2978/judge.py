#!/opt/homebrew/bin/python3

import sys
import random

n = 1000
q = 1500
a = [random.randint(1, n) for i in range(n)]

print(n, q)
for qq in range(q + 1):
    s = input()
    if s.startswith("?"):
        l1, r1, l2, r2 = map(int, s.split()[1:])
        if a[(l1 - 1) : r1] < a[(l2 - 1) : r2]:
            print("1")
        else:
            print("0")
        sys.stdout.flush()
    elif s.startswith("!"):
        l1, r1, l2, r2 = map(int, s.split()[1:])
        ng1 = False
        ng2 = False
        for l in range(n):
            for r in range(l + 1, n + 1):
                if a[l:r] < a[(l1 - 1) : r1]:
                    ng1 = True
                if a[l:r] > a[(l2 - 1) : r2]:
                    ng2 = True
        print(f"q = {qq}/{q}", file=sys.stderr)
        sys.exit(int(ng1 or ng2))
    else:
        sys.exit(1)

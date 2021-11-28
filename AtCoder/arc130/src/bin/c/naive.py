a = list(input())
b = list(input())

import itertools

ans = 10000000000
for pa in itertools.permutations(a):
    for pb in itertools.permutations(b):
        s = int("".join(pa)) + int("".join(pb))
        ss = sum([int(d) for d in str(s)])
        if ans > ss:
            ans = ss
print(ans)

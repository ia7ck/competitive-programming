def mex(a):
    for x in range(len(a)):
        if x not in a:
            return x
    return len(a)

def solve(a):
    rm = max(a)
    a.remove(rm)
    g = set()
    for x in range(rm):
        if x not in a:
            a.add(x)
            g.add(solve(a))
            a.remove(x)
    a.add(rm)
    return mex(g)

# n = int(input())
# a = list(map(int, input().split()))

import random
for _ in range(100):
    a = [random.randint(0, 10) for _ in range(5)]
    a = sorted(list(set(a)))
    print(a, solve(set(a)))

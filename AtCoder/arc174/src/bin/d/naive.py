import math
import collections
import sys

t = int(input())

for _t in range(t):
    n = int(input())
    counter = collections.Counter()
    for x in range(1, n + 1):
        y = int(math.sqrt(x))
        if str(x).startswith(str(y)):
            counter[y] += 1
    # print(sorted(counter.items()), file=sys.stderr)
    print(sum(counter.values()))

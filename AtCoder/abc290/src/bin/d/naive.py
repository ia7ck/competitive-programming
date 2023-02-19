import sys

def solve(n, d, k):
    a = [False] * n
    x = 0
    a[x] = True
    for _ in range(k - 1):
        x = (x + d) % n
        log = []
        while a[x]:
            # log.append(f"{x} -> {x + 1}")
            x = (x + 1) % n
        # if len(log) >= 1:
        #     print("len = {}".format(len(log)), file=sys.stderr)
        #     print(", ".join(log), file=sys.stderr)
        #     assert len(log) == 1
        a[x] = True
    print(x)
    

t = int(input())
for _ in range(t):
    n, d, k = map(int, input().split())
    solve(n, d, k)

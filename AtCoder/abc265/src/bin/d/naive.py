import sys

n, p, q, r = map(int, input().split())
a = list(map(int, input().split()))

for x in range(n):
    for y in range(x + 1, n):
        for z in range(y + 1, n):
            for w in range(z + 1, n + 1):
                if sum(a[x:y]) == p and sum(a[y:z]) == q and sum(a[z:w]) == r:
                    print("Yes")
                    sys.exit(0)
print("No")

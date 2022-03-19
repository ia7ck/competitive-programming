n = int(input())
a = list(map(int, input().split()))

ones = set()
ones.add(len([x for x in a if x == 1]))
for l in range(n):
    for r in range(l, n):
        b = [a[i] ^ 1 if l <= i <= r else a[i] for i in range(n)]
        ones.add(len([x for x in b if x == 1]))
print(len(ones))

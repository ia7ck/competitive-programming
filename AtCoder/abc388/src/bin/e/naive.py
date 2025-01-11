import itertools

n = int(input())
a = list(map(int, input().split()))

ans = 0
for p in itertools.permutations(a):
    count = 0
    for i in range(1, len(p), 2):
        if p[i - 1] <= p[i] // 2:
            count += 1
    if ans < count:
        ans = count

print(ans)

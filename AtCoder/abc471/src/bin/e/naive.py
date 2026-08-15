import itertools

n, k = map(int, input().split())
a = list(map(int, input().split()))

ans = 0
for p in itertools.combinations(a, k):
    ans += sum(p) ** 2
    ans %= 998244353

print(ans)

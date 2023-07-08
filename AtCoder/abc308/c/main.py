from decimal import Decimal

n = int(input())
ab = [list(map(int, input().split())) for _ in range(n)]

data = []
for i in range(n):
    a, b = ab[i]
    data.append((-Decimal(a) / Decimal(a + b), i))

data.sort()
print(*[x[1] + 1 for x in data])

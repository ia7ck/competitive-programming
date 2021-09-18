n = int(input())
x, y = map(int, input().split())
ab = [list(map(int, input().split())) for _ in range(n)]

INF = 10 ** 9
ans = INF
for bits in range(1 << n):
    a = 0
    b = 0
    for i in range(n):
        if bits >> i & 1 == 1:
            a += ab[i][0]
            b += ab[i][1]
    if a >= x and b >= y:
        ans = min(ans, bin(bits).count('1'))
if ans == INF:
    print(-1)
else:
    print(ans)

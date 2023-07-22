h, w, n = map(int, input().split())
hole = [[False] * (w + 1) for _ in range(h + 1)]
for _ in range(n):
    a, b = map(int, input().split())
    hole[a][b] = True

ans = 0
for i in range(1, h + 1):
    for j in range(1, w + 1):
        for n in range(max(h, w)):
            i2 = i + n
            j2 = j + n
            if i2 > h or j2 > w:
                continue
            ok = True
            for y in range(i, i2 + 1):
                for x in range(j, j2 + 1):
                    if hole[y][x]:
                        ok = False
            if ok:
                ans += 1
print(ans)

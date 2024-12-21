n, m, sx, sy = map(int, input().split())
xy = []
for i in range(n):
    x, y = map(int, input().split())
    xy.append((x, y))
dc = []
for i in range(m):
    d, c = input().split()
    dc.append((d, int(c)))

visited = set()
visited.add((sx, sy))
x, y = sx, sy
for d, c in dc:
    for i in range(c):
        if d == 'U':
            y += 1
        elif d == 'D':
            y -= 1
        elif d == 'L':
            x -= 1
        elif d == 'R':
            x += 1
        visited.add((x, y))
ans = 0
for (px, py) in xy:
    if (px, py) in visited:
        ans += 1
print(x, y, ans)

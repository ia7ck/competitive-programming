import random

n = random.randint(1, 4)
m = random.randint(1, 4)
sx = random.randint(-10, 10)
sy = random.randint(-10, 10)
xy = set()

while len(xy) < n:
    x = random.randint(-10, 10)
    y = random.randint(-10, 10)
    xy.add((x, y))

dc = []
for i in range(m):
    d = random.choice("UDLR")
    c = random.randint(1, 10)
    dc.append((d, c))

print(n, m, sx, sy)
for x, y in xy:
    print(x, y)
for d, c in dc:
    print(d, c)

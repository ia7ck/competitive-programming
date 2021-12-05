import sys

a = [l.strip() for l in sys.stdin.readlines()]
h = len(a)
w = len(a[0])
for i in range(len(a)):
    for j in range(len(a[i])):
        ok = True
        if a[i][j] == '.':
            ok = False
        for (di, dj) in ((-1, 0), (0, 1), (1, 0), (0, -1)):
            ni = i + di
            nj = j + dj
            if 0 <= ni < h and 0 <= nj < w:
                if a[ni][nj] == a[i][j]:
                    ok = False
                    break
        if not ok:
            sys.exit(1)


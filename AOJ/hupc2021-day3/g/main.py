from collections import deque


def main():
    h, w = map(int, input().split())
    c = [input() for _ in range(h)]
    sy, sx, gy, gx = 0, 0, 0, 0
    for i in range(h):
        for j in range(w):
            if c[i][j] == 'S':
                sy, sx = i, j
            if c[i][j] == 'G':
                gy, gx = i, j
    inf = 1_000_000_000
    d = [[inf] * w for _ in range(h)]
    d[sy][sx] = 0
    que = deque()
    que.append((sy, sx))
    dy = [-1, 0, 0, 1]
    dx = [0, -1, 1, 0]
    while len(que) >= 1:
        y, x = que.popleft()

        if c[y][x] in "UDLR":
            if c[y][x] == 'U':
                if y >= 1 and c[y - 1][x] != '#':
                    if d[y - 1][x] > d[y][x]:
                        d[y - 1][x] = d[y][x]
                        que.appendleft((y - 1, x))
            elif c[y][x] == 'D':
                if y + 1 < h and c[y + 1][x] != '#':
                    if d[y + 1][x] > d[y][x]:
                        d[y + 1][x] = d[y][x]
                        que.appendleft((y + 1, x))
            elif c[y][x] == 'L':
                if x >= 1 and c[y][x - 1] != '#':
                    if d[y][x - 1] > d[y][x]:
                        d[y][x - 1] = d[y][x]
                        que.appendleft((y, x - 1))
            elif c[y][x] == 'R':
                if x + 1 < w and c[y][x + 1] != '#':
                    if d[y][x + 1] > d[y][x]:
                        d[y][x + 1] = d[y][x]
                        que.appendleft((y, x + 1))
            continue

        for k in range(4):
            ny = y + dy[k]
            nx = x + dx[k]
            if 0 <= ny < h and 0 <= nx < w and c[ny][nx] != '#':
                if d[ny][nx] > d[y][x] + 1:
                    d[ny][nx] = d[y][x] + 1
                    que.append((ny, nx))

    ans = d[gy][gx]
    if ans == inf:
        print(-1)
    else:
        print(ans)


if __name__ == '__main__':
    main()

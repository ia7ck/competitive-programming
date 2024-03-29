from collections import deque


def main():
    h, w = map(int, input().split())
    g = [input() for _ in range(h)]
    inf = 999_999_999
    t = [[inf] * w for _ in range(h)]
    m = int(input())
    for _ in range(m):
        a, b, tt = map(int, input().split())
        t[a - 1][b - 1] = tt

    sy, sx, vy, vx = 0, 0, 0, 0
    for i in range(h):
        for j in range(w):
            if g[i][j] == 'S':
                sy, sx = i, j
            if g[i][j] == 'V':
                vy, vx = i, j

    d = [[inf] * w for _ in range(h)]
    d[sy][sx] = 0
    que = deque()
    que.append((sy, sx))
    dy = [-1, 0, 0, 1]
    dx = [0, -1, 1, 0]
    while len(que) >= 1:
        y, x = que.popleft()
        for k in range(4):
            ny, nx = y + dy[k], x + dx[k]
            if 0 <= ny < h and 0 <= nx < w and g[ny][nx] != '#':
                if d[ny][nx] > d[y][x] + 1 and d[y][x] + 1 < t[ny][nx]:
                    d[ny][nx] = d[y][x] + 1
                    que.append((ny, nx))
    d_s_v = d[vy][vx]
    if d_s_v == inf:
        print("No")
        return

    d = [[inf] * w for _ in range(h)]
    d[vy][vx] = d_s_v
    que = deque()
    que.append((vy, vx))
    while len(que) >= 1:
        y, x = que.popleft()
        for k in range(4):
            ny, nx = y + dy[k], x + dx[k]
            if 0 <= ny < h and 0 <= nx < w and g[ny][nx] != '#':
                if d[ny][nx] > d[y][x] + 1 and d[y][x] + 1 < t[ny][nx]:
                    d[ny][nx] = d[y][x] + 1
                    que.append((ny, nx))
    if d[sy][sx] == inf:
        print("No")
    else:
        print("Yes")


if __name__ == '__main__':
    main()

if __name__ == "__main__":
    R, C = map(int, input().split())
    sy, sx = map(int, input().split())
    gy, gx = map(int, input().split())
    sy, sx = sy - 1, sx - 1
    gy, gx = gy - 1, gx - 1
    a = [input() for _ in range(R)]

    d = [[-1 for _ in range(C)] for _ in range(R)]
    dydx = [(-1, 0), (0, -1), (1, 0), (0, 1)]
    from collections import deque

    q = deque()
    d[sy][sx] = 0
    q.append((sy, sx))
    while len(q) > 0:
        y, x = q.popleft()
        for dy, dx in dydx:
            ny, nx = y + dy, x + dx
            if 0 <= ny < R and 0 <= nx < C:
                if a[ny][nx] == "." and d[ny][nx] == -1:
                    d[ny][nx] = d[y][x] + 1
                    q.append((ny, nx))
    print(d[gy][gx])

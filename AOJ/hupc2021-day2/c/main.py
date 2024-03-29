from heapq import heappush, heappop
from collections import deque


def main():
    h, w, n = map(int, input().split())
    sx, sy, gx, gy = map(int, input().split())
    sx -= 1
    sy -= 1
    gx -= 1
    gy -= 1
    block = [[False] * w for _ in range(h)]
    heap = []
    for _ in range(n):
        x, y, k = map(int, input().split())
        block[x - 1][y - 1] = True
        if k >= 1:
            heappush(heap, (-k, x - 1, y - 1))
    dx = [0, -1, 1, 0]
    dy = [-1, 0, 0, 1]
    while len(heap) >= 1:
        k, x, y = heappop(heap)
        k = -k
        for i in range(4):
            nx, ny = x + dx[i], y + dy[i]
            if 0 <= nx < h and 0 <= ny < w:
                if not block[nx][ny]:
                    block[nx][ny] = True
                    if k - 1 >= 1:
                        heappush(heap, (-(k - 1), nx, ny))

    visited = [[False] * w for _ in range(h)]
    visited[sx][sy] = True
    que = deque()
    que.append((sx, sy))
    while len(que) >= 1:
        x, y = que.popleft()
        for i in range(4):
            nx, ny = x + dx[i], y + dy[i]
            if 0 <= nx < h and 0 <= ny < w and (not block[nx][ny]):
                if not visited[nx][ny]:
                    visited[nx][ny] = True
                    que.append((nx, ny))
    if visited[gx][gy]:
        print("Yes")
    else:
        print("No")


if __name__ == '__main__':
    main()

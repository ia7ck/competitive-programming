from collections import deque

n, m = map(int, input().split())

ans = [[-1] * n for _ in range(n)]
que = deque()
ans[0][0] = 0
que.append([0, 0])
while len(que) > 0:
    [i, j] = que.popleft()
    for y in range(n):
        for x in range(n):
            if (i - y) ** 2 + (j - x) ** 2 == m:
                if ans[y][x] == -1:
                    ans[y][x] = ans[i][j] + 1
                    que.append([y, x])

for row in ans:
    print(*row)

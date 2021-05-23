def main():
    h, w = map(int, input().split())
    s = [list(input()) for _ in range(h)]
    empty_cells = []
    for i in range(h):
        for j in range(w):
            if s[i][j] == '.':
                empty_cells.append((i, j))
    ans = 0
    for bits in range(1 << len(empty_cells)):
        for i, (r, c) in enumerate(empty_cells):
            if bits >> i & 1:
                s[r][c] = 'R'
            else:
                s[r][c] = 'B'
        max_dp = [[0] * w for _ in range(h)]
        min_dp = [[2525] * w for _ in range(h)]
        if s[0][0] == 'R':
            max_dp[0][0] = 1
            min_dp[0][0] = 1
        else:
            max_dp[0][0] = 0
            min_dp[0][0] = 0
        for i in range(h):
            for j in range(w):
                if i + 1 < h:
                    if s[i + 1][j] == 'R':
                        max_dp[i + 1][j] = max(max_dp[i + 1][j], max_dp[i][j] + 1)
                        min_dp[i + 1][j] = min(min_dp[i + 1][j], min_dp[i][j] + 1)
                    else:
                        max_dp[i + 1][j] = max(max_dp[i + 1][j], max_dp[i][j])
                        min_dp[i + 1][j] = min(min_dp[i + 1][j], min_dp[i][j])
                if j + 1 < w:
                    if s[i][j + 1] == 'R':
                        max_dp[i][j + 1] = max(max_dp[i][j + 1], max_dp[i][j] + 1)
                        min_dp[i][j + 1] = min(min_dp[i][j + 1], min_dp[i][j] + 1)
                    else:
                        max_dp[i][j + 1] = max(max_dp[i][j + 1], max_dp[i][j])
                        min_dp[i][j + 1] = min(min_dp[i][j + 1], min_dp[i][j])
        if max_dp[h - 1][w - 1] == min_dp[h - 1][w - 1]:
            ans += 1
    print(ans)


if __name__ == '__main__':
    main()

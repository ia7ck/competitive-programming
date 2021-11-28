def main():
    h, w, C, q = map(int, input().split())
    field = [[0] * w for _ in range(h)]
    for _ in range(q):
        t, n, c = map(int, input().split())
        if t == 1:
            for j in range(w):
                field[n - 1][j] = c
        else:
            for i in range(h):
                field[i][n - 1] = c
    ans = []
    for x in range(1, C + 1):
        count = 0
        for i in range(h):
            for j in range(w):
                if field[i][j] == x:
                    count += 1
        ans.append(count)
    print(*ans)

if __name__ == '__main__':
    main()

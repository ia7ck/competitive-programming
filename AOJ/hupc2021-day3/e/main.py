def main():
    n = int(input())
    a = [list(map(int, input().split())) for _ in range(n)]

    ul = {}
    for i in range(n):
        for j in range(n):
            if a[i][j] not in ul:
                ul[a[i][j]] = (i, j)

    q = int(input())
    for _ in range(q):
        t = int(input())
        if t in ul:
            i, j = ul[t]
            print(i + 1, j + 1)
        else:
            print(-1)


if __name__ == '__main__':
    main()

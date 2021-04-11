def main():
    n = int(input())
    colors = list(map(int, input().split()))
    g = [[] for _ in range(n)]
    for _ in range(n - 1):
        a, b = map(int, input().split())
        g[a - 1].append(b - 1)
        g[b - 1].append(a - 1)
    par = [0] * n

    def dfs(i, p):
        par[i] = p
        for j in g[i]:
            if j != p:
                dfs(j, i)

    dfs(0, -1)
    for i in range(n):
        c = colors[i]
        p = par[i]
        ok = True
        while p >= 0:
            if c == colors[p]:
                ok = False
                break
            p = par[p]
        if ok:
            print(i + 1)


if __name__ == '__main__':
    main()

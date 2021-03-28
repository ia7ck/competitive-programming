ans = 0


def dfs(n: int, m: int, a: [int]):
    global ans
    if len(a) == n:
        ans += 1
        return
    last = a[-1]
    for x in range(last, m + 1):
        if x % last == 0:
            dfs(n, m, a + [x])


def main():
    n, m = map(int, input().split())
    for i in range(1, m + 1):
        dfs(n, m, [i])
    print(ans)


if __name__ == '__main__':
    main()

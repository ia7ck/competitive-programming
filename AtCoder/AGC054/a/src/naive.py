inf = 1_000_000_000
ans = inf


def solve(s: str, acc: int):
    global ans
    n = len(s)
    if n == 0:
        ans = min(ans, acc)
        return
    if n == 1:
        return
    for i in range(0, n - 1):
        for j in range(i + 1, n):
            if s[i] != s[j]:
                t = s[:i] + s[(j + 1):]
                solve(t, acc + 1)


def main():
    n = int(input())
    s = input()

    solve(s, 0)
    if ans == inf:
        print(-1)
    else:
        print(ans)


if __name__ == '__main__':
    main()

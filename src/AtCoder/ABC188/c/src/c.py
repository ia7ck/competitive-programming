def solve(ai: [(int, int)]):
    if len(ai) == 2:
        (_, ans) = min(ai[0], ai[1])
        print(ans + 1)
        exit(0)
    bi = []
    for i in range(0, len(ai), 2):
        bi.append(max(ai[i], ai[i + 1]))
    solve(bi)


def main():
    n = int(input())
    a = list(map(int, input().split()))

    ai = []
    for i in range(1 << n):
        ai.append((a[i], i))
    solve(ai)


if __name__ == '__main__':
    main()

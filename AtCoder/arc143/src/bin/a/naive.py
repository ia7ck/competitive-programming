INF = 1_000_000_000

def solve(a, b, c):
    if a == 0 and b == 0 and c == 0:
        return 0
    result = INF
    if a >= 1 and b >= 1:
        result = min(result, 1 + solve(a - 1, b - 1, c))
    if b >= 1 and c >= 1:
        result = min(result, 1 + solve(a, b - 1, c - 1))
    if c >= 1 and a >= 1:
        result = min(result, 1 + solve(a - 1, b, c - 1))
    if a >= 1 and b >= 1 and c >= 1:
        result = min(result, 1 + solve(a - 1, b - 1, c - 1))
    return result

def main():
    a, b, c = map(int, input().split())
    ans = solve(a, b, c)
    if ans == INF:
        print(-1)
    else:
        print(ans)


if __name__ == '__main__':
    main()

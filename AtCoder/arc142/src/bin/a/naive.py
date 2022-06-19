def reverse(n):
    m = 0
    while n > 0:
        m = m * 10 + n % 10
        n //= 10
    return m

def f(x):
    y = reverse(x)
    return min(x, y, reverse(y))

def main():
    n, k = map(int, input().split())

    ans = 0
    for x in range(1, n + 1):
        if f(x) == k:
            ans += 1
    print(ans)


if __name__ == '__main__':
    main()

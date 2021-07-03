def main():
    a, b, c, d = map(int, input().split())
    red = a
    blue = 0
    ans = 0
    while red > blue * d:
        if ans >= 100000:
            print(-1)
            return
        red += b
        blue += c
        ans += 1
    print(ans)


if __name__ == '__main__':
    main()

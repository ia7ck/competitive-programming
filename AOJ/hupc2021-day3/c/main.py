def main():
    n, x = map(int, input().split())
    m = [int(input()) for _ in range(n)]

    ans = len([y for y in m if y >= x])
    print(ans)


if __name__ == '__main__':
    main()

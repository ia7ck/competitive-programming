def main():
    x = int(input())
    if x % 100 == 0:
        print(100)
        return
    ans = 0
    while x % 100 != 0:
        x += 1
        ans += 1
    print(ans)


if __name__ == '__main__':
    main()

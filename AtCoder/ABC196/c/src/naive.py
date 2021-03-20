def main():
    n = int(input())
    ans = 0
    for i in range(1, n + 1):
        s = str(i)
        if len(s) % 2 == 1:
            continue
        if s[:len(s) // 2] == s[len(s) // 2:]:
            ans += 1
    print(ans)


if __name__ == '__main__':
    main()

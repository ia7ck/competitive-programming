def main():
    l, n = map(int, input().split())
    a = list(map(int, input().split()))
    for i in range(n):
        if sum(a[:(i + 1)]) >= l:
            print(i + 1)
            return
    print("No")


if __name__ == '__main__':
    main()

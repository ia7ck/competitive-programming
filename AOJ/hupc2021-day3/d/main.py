def main():
    n, m, k, x = map(int, input().split())
    a = list(map(int, input().split()))

    # a/m * k >= x
    max_a = max(a)
    if max_a * k >= x * m:
        print("Yes")
    else:
        print("No")


if __name__ == '__main__':
    main()

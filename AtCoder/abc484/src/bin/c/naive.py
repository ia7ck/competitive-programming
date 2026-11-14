def main():
    n = int(input())
    a = list(map(int, input().split()))
    b = list(map(int, input().split()))

    for i in range(n):
        y = a[i]
        a0 = a[:i] + a[(i + 1) :]
        for j in range(n):
            a1 = a0[:j] + [y] + a0[j:]
            if a1 == b:
                print("Yes")
                return

    print("No")


if __name__ == "__main__":
    main()

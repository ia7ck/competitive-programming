import collections


def main():
    n, k = map(int, input().split())
    a = list(map(int, input().split()))

    # 1 -> 5 -> 6
    # 2 -> 6
    # 3 -> 6
    # 4 -> 6
    # 5 -> 6
    # 6

    ans = 0

    for i in range(n):
        if a[i] == 1 and k >= 1:
            a[i] = 5
            k -= 1

    for i in range(n):
        if a[i] == 2 and k >= 1:
            a[i] = 6
            k -= 1

    for i in range(n):
        if a[i] == 3 and k >= 1:
            a[i] = 6
            k -= 1

    for i in range(n):
        if a[i] == 4 and k >= 1:
            a[i] = 6
            k -= 1

    for i in range(n):
        if a[i] == 5 and k >= 1:
            a[i] = 6
            k -= 1

    ans = sum(a)
    print(ans)


if __name__ == '__main__':
    main()

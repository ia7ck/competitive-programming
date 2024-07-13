def check(a):
    if len(a) <= 2:
        return True
    d = a[1] - a[0]
    for i in range(2, len(a)):
        if a[i] - a[i - 1] != d:
            return False
    return True

def main():
    n = int(input())
    a = list(map(int, input().split()))

    ans = [0] * (n + 1)
    for bits in range(1 << n):
        b = [a[i] for i in range(n) if bits >> i & 1 == 1]
        if check(b):
            ans[len(b)] += 1

    print(*ans[1:])


if __name__ == '__main__':
    main()

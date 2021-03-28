def main():
    n = int(input())
    a = list(map(int, input().split()))
    ans = 0
    for bits in range(1, 1 << n):
        b = [a[i] for i in range(n) if bits >> i & 1]
        ans += max(b) * min(b)
        ans %= 998244353
    print(ans)


if __name__ == '__main__':
    main()

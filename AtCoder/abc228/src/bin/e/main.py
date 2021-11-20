def main():
    n, k, m = map(int, input().split())
    p = 998244353
    if m % p == 0:
        print(0)
        return
    ans = pow(m, pow(k, n, p - 1), p)
    print(ans)

if __name__ == '__main__':
    main()

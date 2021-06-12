def main():
    n = int(input())
    a = list(map(int, input().split()))
    ans = 0
    for bits in range(1 << n):
        if bits & 1 == 0:
            continue
        ok = True
        for i in range(1, n):
            if bits >> (i - 1) & 1 == 0 and bits >> i & 1 == 0:
                ok = False
                break
        if ok:
            for i in range(n):
                if bits >> i & 1 == 1:
                    ans += a[i]
                else:
                    ans -= a[i]
                ans %= 1_000_000_000 + 7;
    print(ans)

if __name__ == '__main__':
    main()

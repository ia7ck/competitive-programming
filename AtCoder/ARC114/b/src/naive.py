def main():
    n = int(input())
    a = list(map(int, input().split()))
    a = [x - 1 for x in a]
    ans = 0
    for bits in range(1 << n):
        u = [i for i in range(n) if bits >> i & 1]
        if any([a[x] not in u for x in u]):
            continue
        ng = False
        for i in range(len(u)):
            for j in range(i):
                if u[i] != u[j] and a[u[i]] == a[u[j]]:
                    ng = True
        if not ng:
            ans += 1
    print(ans - 1)


if __name__ == '__main__':
    main()

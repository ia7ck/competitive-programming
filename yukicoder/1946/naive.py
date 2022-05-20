def main():
    n, m = map(int, input().split())
    a = list(map(int, input().split()))

    assert n <= 10

    for bits in range(1 << n):
        op = [False] * (n + 1)
        for i in range(n):
            if bits >> i & 1:
                for d in range(1, i + 2):
                    if (i + 1) % d == 0:
                        op[d] = not op[d]
            b = [i for i in range(1, n + 1) if op[i]]
            if a == b:
                print(n - bin(bits).count('1'))
                return
    assert False

if __name__ == '__main__':
    main()

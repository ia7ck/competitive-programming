def main():
    t, n = map(int, input().split())
    ok = set()
    for x in range(1, 1_000_000):
        ok.add(x * (100 + t) // 100)
    for x in range(1, 1_000_000):
        if x not in ok:
            n -= 1
            if n == 0:
                print(x)
                return

    assert False


if __name__ == '__main__':
    main()

import itertools

def main():
    n, m = map(int, input().split())
    ab = [list(map(int, input().split())) for _ in range(m)]

    for p in itertools.permutations(range(n)):
        ok = True
        for [a, b] in ab:
            found = False
            for i in range(n - 1):
                if (p[i] == a - 1 and p[i + 1] == b - 1) or (p[i] == b - 1 and p[i + 1] == a - 1):
                    found = True
                    break
            if not found:
                ok = False
                break
        if ok:
            print("Yes")
            return
    print("No")

if __name__ == '__main__':
    main()

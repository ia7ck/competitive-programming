import itertools

def main():
    n, k = map(int, input().split())
    for p in itertools.permutations(range(n)):
        ok = True
        for i in range(n):
            if abs((p[i] + 1) - (i + 1)) < k:
                ok = False
                break
        if ok:
            print(*[x + 1 for x in p])
            return
    print(-1)

if __name__ == '__main__':
    main()

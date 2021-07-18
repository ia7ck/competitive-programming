import itertools


def main():
    n = int(input())
    a = list(map(int, input().split()))
    b = list(map(int, input().split()))
    c = list(map(int, input().split()))
    ans = 0
    for p in itertools.permutations(a):
        for q in itertools.permutations(b):
            for r in itertools.permutations(c):
                f = 0
                for i in range(n):
                    if p[i] < q[i] < r[i]:
                        f += 1
                ans = max(ans, f)
    print(ans)


if __name__ == '__main__':
    main()

import itertools


def main():
    n = int(input())
    ac = []
    for _ in range(n * 2):
        l = input().split()
        ac.append((int(l[0]), l[1]))
    ans = 123456789
    for p in itertools.permutations(range(n * 2)):
        cost = 0
        for i in range(n):
            (a, c) = ac[p[i * 2]]
            (aa, cc) = ac[p[i * 2 + 1]]
            if c != cc:
                cost += abs(a - aa)
                if cost >= ans:
                    break
        if cost < ans:
            ans = cost
    print(ans)


if __name__ == '__main__':
    main()

import sys
sys.setrecursionlimit(1_000_00)

memo = set()


def dfs(a: [int]):
    key = " ".join(map(str, a))
    if key in memo:
        return
    memo.add(key)
    mx = max(a)
    for x in range(1, mx + 1):
        dfs([y if y < x else y - 1 for y in a])


def main():
    n = int(input())
    a = list(map(int, input().split()))
    dfs(a)
    print(len(memo))
    # b = [s.split(" ") for s in memo]
    # b = [sorted(list(map(int, result))) for result in b]
    # print(sorted(b))


if __name__ == '__main__':
    main()

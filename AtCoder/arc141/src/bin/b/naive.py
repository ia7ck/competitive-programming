from collections import defaultdict

answer = 0
freq_by_last = defaultdict(int)
freq_by_first = defaultdict(int)

def dfs(n, m, a):
    if n == len(a):
        b = a.copy()
        for i in range(1, n):
            b[i] = b[i] ^ b[i - 1]
        ok = True
        for i in range(1, n):
            assert a[i - 1] < a[i]
            if b[i - 1] >= b[i]:
                ok = False
        if ok:
            global answer
            global freq_by_last
            global freq_by_first
            answer += 1
            freq_by_last[a[-1]] += 1
            freq_by_first[a[0]] += 1
            # print("a = ", a)
            # print("b = ", b)
        else:
            # print(a)
            pass
        return
    begin = 0 if len(a) == 0 else a[-1]
    for x in range(begin + 1, m + 1):
        a.append(x)
        dfs(n, m, a)
        a.pop()


def main():
    n, m = map(int, input().split())

    assert n <= 6
    assert m <= 35

    dfs(n, m, [])
    print(answer)
    if False:
        for k, v in freq_by_last.items():
            print("last = {}, freq = {}".format(k, v))
        for k, v in freq_by_first.items():
            print("first = {}, freq = {}".format(k, v))

if __name__ == '__main__':
    main()


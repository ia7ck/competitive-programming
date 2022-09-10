import itertools

def calc(s):
    x = 0
    f = [0] * 10
    total = 0
    for ch in s:
        if ch == 'X':
            x += 1
        else:
            d = ord(ch) - ord('0')
            f[d] += 1
            total += d * x
    return total

n = int(input())
ss = [input() for _ in range(n)]

ans = 0
for p in itertools.permutations(range(n)):
    s = ""
    for i in p:
        s += ss[i]
    ans = max(ans, calc(s))
print(ans)

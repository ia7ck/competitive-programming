def calc(s, t):
    n = min(len(s), len(t))
    for i in range(n):
        if s[i] != t[i]:
            return i
    return n

n = int(input())
s = [input() for _ in range(n)]

for i in range(n):
    ans = 0
    for j in range(n):
        if i != j:
            ans = max(ans, calc(s[i], s[j]))
    print(ans)

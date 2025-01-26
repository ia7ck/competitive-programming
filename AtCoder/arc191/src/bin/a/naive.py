n, m = map(int, input().split())
s = input()
t = input()

def dfs(S, k):
    if k == m:
        return S[:]
    res = []
    for i in range(n):
        old = S[i]
        S[i] = t[k]
        res.append(dfs(S, k + 1))
        S[i] = old
    return max(res)

ans = dfs(list(s), 0)
print("".join(ans))

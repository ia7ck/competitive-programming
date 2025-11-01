n, a, b = map(int, input().split())
s = input()

ans = 0
for L in range(n):
    for R in range(L, n):
        if s[L : R + 1].count("a") >= a and s[L : R + 1].count("b") < b:
            ans += 1
print(ans)

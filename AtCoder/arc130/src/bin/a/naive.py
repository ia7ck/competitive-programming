n = int(input())
s = input()

ans = 0
for i in range(n):
    for j in range(i + 1, n):
        si = s[:i] + s[i + 1:]
        sj = s[:j] + s[j + 1:]
        if si == sj:
            ans += 1
print(ans)

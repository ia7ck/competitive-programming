s = list(input())

ans = set()
for i in range(len(s)):
    for j in range(i + 1, len(s)):
        t = s.copy()
        t[i], t[j] = t[j], t[i]
        ans.add("".join(t))
print(len(ans))


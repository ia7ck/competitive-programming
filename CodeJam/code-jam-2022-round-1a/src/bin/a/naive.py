t = int(input())
assert t == 1
s = input()
ans = s
for bits in range(1 << len(s)):
    t = ""
    for i in range(len(s)):
        if bits >> i & 1:
            t += s[i]
            t += s[i]
        else:
            t += s[i]
    ans = min(ans, t)
print("Case #1: {}".format(ans))

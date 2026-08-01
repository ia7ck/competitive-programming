n = int(input())
s = input()

for k in range(n):
    ans = k + 1
    o = s[: k + 1].count("o")
    i = k + 1
    while o > 0 and i < n:
        o -= 1
        if s[i] == "o":
            o += 1
        i += 1
        ans += 1
    print(ans)

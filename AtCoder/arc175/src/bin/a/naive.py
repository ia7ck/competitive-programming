n = int(input())
p = list(map(int, input().split()))
s = input()

ans = 0
for bits in range(1 << n):
    t = "".join("L" if bits >> i & 1 else "R" for i in range(n))
    ok = True
    for i in range(n):
        if s[i] == "?":
            continue
        if s[i] != t[i]:
            ok = False
            break
    if not ok:
        continue
    spoon = [True] * n
    for i in p:
        i -= 1
        if spoon[i] and spoon[(i + 1) % n]:
            if t[i] == "L":
                spoon[i] = False
            else:
                spoon[(i + 1) % n] = False
        elif spoon[i] and (not spoon[(i + 1) % n]):
            spoon[i] = False
        elif (not spoon[i]) and spoon[(i + 1) % n]:
            spoon[(i + 1) % n] = False
        else:
            ok = False
            break
    if ok:
        print(t)
        ans += 1
print(ans)
    
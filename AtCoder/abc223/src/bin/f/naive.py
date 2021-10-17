n, q = map(int, input().split())
s = list(input())
for _ in range(q):
    t, l, r = map(int, input().split())
    if t == 1:
        s[l - 1], s[r - 1] = s[r - 1], s[l - 1]
    else:
        c = 0
        ok = True
        for i in range(l - 1, r):
            if s[i] == '(':
                c += 1
            else:
                c -= 1
            if c < 0:
                ok = False

        if c == 0 and ok:
            print("Yes")
        else:
            print("No")

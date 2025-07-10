n, q = map(int, input().split())
s = input()

for _q in range(q):
    l, r = map(int, input().split())
    ans = 0
    for i in range(l - 1, r):
        if s[i] == '/':
            left_one = s[l - 1:i].count("1")
            right_two = s[i:r].count("2")
            ans = max(ans, 1 + min(left_one, right_two) * 2)
    print(ans)

def balanced(s):
    op = 0
    for ch in s:
        if ch == "(":
            op += 1
        else:
            op -= 1
        if op < 0:
            return False
    return op == 0


q = int(input())
s = []
for _ in range(q):
    query = input().split()
    if query[0] == "1":
        s.append(query[1])
    else:
        s.pop()
    if balanced(s):
        print("Yes")
    else:
        print("No")

import random


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


q = random.randint(1, 10)
print(q)
s = []
for _ in range(q):
    c = random.choice("()")
    if len(s) == 0:
        s.append(c)
        print(1, c)
    else:
        t = random.randint(1, 2)
        if t == 1:
            s.append(c)
            print(1, c)
        else:
            s.pop()
            print(2)

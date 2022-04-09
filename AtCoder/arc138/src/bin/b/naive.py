n = int(input())
a = list(map(int, input().split()))

valid_pattern = []

def f(a):
    if len(a) == n:
        valid_pattern.append(a)
        return
    f([0] + [x ^ 1 for x in a])
    f([x for x in a] + [0])

f([])

if a in valid_pattern:
    print("Yes")
else:
    print("No")

# valid
# 
# 0, 0, 0, 0, 0, 0
# 0, 1, 0, 0, 0, 0
# 0, 1, 0, 0, 0, 1
# 0, 1, 0, 0, 1, 0
# 0, 1, 0, 0, 1, 1
# 0, 1, 0, 1, 0, 0
# 0, 1, 0, 1, 0, 1
# 0, 1, 0, 1, 1, 0
# 0, 1, 0, 1, 1, 1
# 0, 1, 1, 0, 0, 0
# 0, 1, 1, 1, 0, 0
# 0, 1, 1, 1, 1, 0
# 0, 1, 1, 1, 1, 1

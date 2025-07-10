import random

n = random.randint(1, 30)
q = 1
print(n, q)
s = [random.choice("12/") for i in range(n)]
print("".join(s))
l = random.randint(1, n)
r = random.randint(1, n)
print(min(l, r), max(l, r))

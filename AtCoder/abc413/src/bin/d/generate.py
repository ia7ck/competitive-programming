import random

t = 1
n = random.randint(2, 5)
a = []
while len(a) < n:
    x = random.randint(-5, 5)
    if x != 0:
        a.append(x)

print(t)
print(n)
print(*a)
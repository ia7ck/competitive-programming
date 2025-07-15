import random

q = 10
k = random.randint(0, 10)
print(q, k)
s = set()
for _i in range(q):
    if len(s) >= 1 and random.randint(0, 1):
        x = random.choice(list(s))
        print(2, x)
    else:
        x = random.randint(1, 10)
        if x in s:
            s.remove(x)
        else:
            s.add(x)
        print(1, x)
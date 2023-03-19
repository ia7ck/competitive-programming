import random

def rl(a):
    b = []
    y = a[0]
    l = 1
    for x in a[1:]:
        if x == y:
            l += 1
        else:
            b.append((y, l))
            y = x
            l = 1
    b.append((y, l))
    total = 0
    for (_, l) in b:
        total += l
    assert total == len(a)
    return b

l = random.randint(1, 10)
a = [random.randint(1, 6) for _ in range(l)]
b = [random.randint(1, 6) for _ in range(l)]
aa = rl(a)
bb = rl(b)
print(l, len(aa), len(bb))
for xx, ll in aa:
    print(xx, ll)
for xx, ll in bb:
    print(xx, ll)

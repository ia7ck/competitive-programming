import random
n = random.randint(2, 10)
q = 5
print(n, q)
s = [random.choice(["(", ")"]) for _ in range(n)]
print("".join(s))
for _ in range(q):
    t = random.randint(1, 2)
    l = random.randint(1, n - 1)
    r = random.randint(l, n)
    print(t, l, r)

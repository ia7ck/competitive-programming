import random

n = random.randint(1, 20)
m = random.randint(1, 5)
s = [random.choice("ABCD") for _ in range(n)]
t = [random.choice("ABCD") for _ in range(m)]
print(n, m)
print("".join(s))
print("".join(t))

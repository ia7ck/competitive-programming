import random

n = random.randint(1, 6)
m = random.randint(1, 6)
s = [str(random.randint(1, 9)) for i in range(n)]
t = [str(random.randint(1, 9)) for i in range(m)]

print(n, m)
print("".join(s))
print("".join(t))

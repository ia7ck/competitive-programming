import random

n = random.randint(1, 10)
k = random.randint(1, 10)
print(n, k)
for _ in range(n):
    a = [str(random.randint(0, 1)) for __ in range(n)]
    print(" ".join(a))
print(1)
s = random.randint(1, n * k - 1)
t = random.randint(s + 1, n * k)
print(s, t)

import random

n = random.randint(1, 10)
a = random.randint(1, n)
b = random.randint(1, n)
s = [random.choice("ab") for _ in range(n)]

print(n, a, b)
print("".join(s))

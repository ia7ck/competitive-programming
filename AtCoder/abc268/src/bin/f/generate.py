import random

n = random.randint(1, 6)
print(n)
for _ in range(n):
    l = random.randint(1, 5)
    s = [random.choice("XXXX123456789") for _ in range(l)]
    print("".join(s))

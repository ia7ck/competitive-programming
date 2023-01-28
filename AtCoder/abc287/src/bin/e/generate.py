import random

n = random.randint(2, 10)
print(n)
for _ in range(n):
    l = random.randint(1, 10)
    s = [random.choice("abcd") for _ in range(l)]
    print("".join(s))

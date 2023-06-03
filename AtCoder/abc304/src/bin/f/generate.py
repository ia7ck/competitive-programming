import random

n = random.randint(1, 20)
s = [random.choice(".#") for _ in range(n)]
print(n)
print("".join(s))

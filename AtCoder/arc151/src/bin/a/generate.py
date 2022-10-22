import random

n = random.randint(1, 10)
s = [random.choice("01") for _ in range(n)]
t = [random.choice("01") for _ in range(n)]
print(n)
print("".join(s))
print("".join(t))

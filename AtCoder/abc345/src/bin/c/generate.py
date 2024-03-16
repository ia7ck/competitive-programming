import random

n = random.randint(2, 10)
s = [random.choice("abcdef") for _ in range(n)]

print("".join(s))

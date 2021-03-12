import random

n = random.randint(5, 15)
k = random.randint(1, n)
s = [random.choice("abcde") for _ in range(n)]
print(n, k)
print("".join(s))

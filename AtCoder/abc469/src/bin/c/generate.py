import random

n = random.randint(1, 10)
s = [random.choice("ox") for i in range(n)]

print(n)
print("".join(s))

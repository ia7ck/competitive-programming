import random

n = random.randint(1, 100)
s = [str(random.randint(1, 9)) for i in range(n)]

print(n)
print("".join(s))

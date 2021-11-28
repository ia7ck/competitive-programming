import random

a = random.randint(1, 10000)
b = random.randint(1, 10000)
while str(a).count('0') >= 1 or str(b).count('0') >= 1:
    a = random.randint(1, 1000)
    b = random.randint(1, 1000)
print(a)
print(b)

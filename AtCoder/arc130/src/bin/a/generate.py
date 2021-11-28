import random

n = random.randint(2, 10)
alph = ['a', 'b', 'c']
s = ""
for _ in range(n):
    s += random.choice(alph)
print(n)
print(s)

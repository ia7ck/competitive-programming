import random
chars = ['a', 'b', 'c', 'd']
n = random.randint(1, 4)
s = ''
for _ in range(n):
    s += random.choice(chars)
print(s)

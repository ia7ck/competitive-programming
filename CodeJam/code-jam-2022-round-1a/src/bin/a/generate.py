import random

print(1)
chars = ['a', 'b', 'c', 'd', 'e']
n = random.randint(1, 10)
s = [random.choice(chars) for _ in range(n)]
print("".join(s))

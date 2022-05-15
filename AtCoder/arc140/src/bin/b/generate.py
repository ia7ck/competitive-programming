import random

n = random.randint(10, 16)
s = [random.choice(['A', 'R', 'C']) for _ in range(n)]

print(n)
print("".join(s))

import random

n = 3
print(n)
for _ in range(n * 2):
    a = random.randint(1, 10)
    c = random.choice(['R', 'G', 'B'])
    print(a, c)

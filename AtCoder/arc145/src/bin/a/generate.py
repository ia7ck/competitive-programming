import random

n = random.randint(2, 7)
print(n)
print("".join([random.choice(['A', 'B']) for _ in range(n)]))

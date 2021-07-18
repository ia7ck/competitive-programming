import random

n = random.randint(1, 5)
print(n)
for _ in range(3):
    a = [str(random.randint(1, 9)) for _ in range(n)]
    print(" ".join(a))

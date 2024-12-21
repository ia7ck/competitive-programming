import random

n = random.randint(1, 10)
h = [random.randint(1, 10) for i in range(n)]
print(n)
print(*h)

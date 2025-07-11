import random

n = random.randint(1, 20)
a = [random.randint(1, n) for i in range(n)]
print(n)
print(*a)

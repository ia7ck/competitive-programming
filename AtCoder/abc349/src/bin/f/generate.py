import random

n = random.randint(1, 5)
m = random.randint(1, 9)
a = [random.randint(1, 9) for i in range(n)]
print(n, m)
print(*a)

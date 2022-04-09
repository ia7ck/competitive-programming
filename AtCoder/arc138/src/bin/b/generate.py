import random
n = random.randint(1, 7)
a = [random.randint(0, 1) for _ in range(n)]
print(n)
print(*a)

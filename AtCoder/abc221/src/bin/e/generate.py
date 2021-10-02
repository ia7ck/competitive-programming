import random
n = random.randint(2, 10)
print(n)
a = [random.randint(1, 9) for _ in range(n)]
print(*a)

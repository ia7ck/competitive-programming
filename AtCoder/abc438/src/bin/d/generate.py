import random

n = random.randint(3, 10)
a = [random.randint(1, 10) for i in range(n)]
b = [random.randint(1, 10) for i in range(n)]
c = [random.randint(1, 10) for i in range(n)]

print(n)
print(*a)
print(*b)
print(*c)

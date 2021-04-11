import random

n = random.randint(2, 10)
c = [random.randint(1, 10) for _ in range(n)]
print(n)
print(*c)
for i in range(n - 1):
    a = i + 1
    b = random.randint(a + 1, n)
    print(a, b)

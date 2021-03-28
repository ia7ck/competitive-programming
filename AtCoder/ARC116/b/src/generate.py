import random

n = random.randint(1, 10)
a = [random.randint(0, 998244353) for _ in range(n)]
print(n)
print(*a)

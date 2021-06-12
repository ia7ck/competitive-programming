import random

n = random.randint(1, 10)
print(n)
a = [str(random.randint(1, 10)) for _ in range(n)]
print(" ".join(a))

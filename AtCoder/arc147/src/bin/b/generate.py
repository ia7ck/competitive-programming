import random

n = random.randint(2, 10)
print(n)
p = [i + 1 for i in range(n)]
random.shuffle(p)
print(*p)

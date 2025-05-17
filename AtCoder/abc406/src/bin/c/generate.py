import random

n = random.randint(4, 20)
p = [i for i in range(1, n + 1)]
random.shuffle(p)

print(n)
print(*p)

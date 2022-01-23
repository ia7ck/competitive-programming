import random
# n = random.randint(1, 4)
n = 8
print(n)
# cand = [pow(2, i) for i in range(16)]
cand = [i for i in range(16)]
for i in range(n * 2 - 1):
    row = [random.choice(cand) for _ in range(n * 2 - i - 1)]
    print(*row)

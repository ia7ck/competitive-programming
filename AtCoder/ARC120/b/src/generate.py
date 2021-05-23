import random

h = random.randint(1, 4)
w = random.randint(1, 4)
print(h, w)
chars = ['R', 'B', '.']
for _ in range(h):
    print("".join([random.choice(chars) for _ in range(w)]))


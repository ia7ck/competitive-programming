import random

n = random.randint(1, 10)
w = random.randint(1, 30)
print(n, w)
for i in range(n):
    w_ = random.randint(1, w)
    v_ = random.randint(1, 100)
    print(w_, v_)

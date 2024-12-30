import random

k = random.randint(1, 20)
s = "".join([random.choice("ab") for i in range(random.randint(1, 4))])
t = "".join([random.choice("ab") for i in range(random.randint(1, 4))])

print(k)
print(s)
print(t)

import random

n = random.randint(2, 10)
alphabet = list("abcdefghijklmn")

s = ""
for _ in range(n):
    s += random.choice(alphabet)
print(n)
print(s)

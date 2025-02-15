import random

n = random.randint(1, 10)
s =[random.choice(['0', '1']) for _ in range(n)]
s[random.randint(0, n - 1)] = '1'

print(n)
print(''.join(s))

n = int(input())
s = input()

ones = []
for i in range(n):
    if s[i] == '1':
        ones.append(i)
ans = float('inf')
for l in range(n):
    r = l + len(ones)
    if r > n:
        break
    cost = 0
    for i in range(l, r):
        cost += abs(ones[i - l] - i)
    ans = min(ans, cost)
    
print(ans)

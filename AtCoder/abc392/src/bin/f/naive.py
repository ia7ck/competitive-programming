n = int(input())
p = list(map(int, input().split()))

a = []
for i in range(n):
    front = a[:p[i]-1]
    back = a[p[i]-1:]
    a = front + [i+1] + back
print(*a)

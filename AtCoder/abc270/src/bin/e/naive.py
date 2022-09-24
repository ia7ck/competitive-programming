n, k = map(int, input().split())
a = list(map(int, input().split()))

i = 0
while k > 0:
    if a[i] > 0:
        k -= 1
        a[i] -= 1
    i = (i + 1) % n
print(*a)

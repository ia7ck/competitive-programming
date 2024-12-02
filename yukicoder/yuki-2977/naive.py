n, k = map(int, input().split())
a = list(map(int, input().split()))

values = []
for i in range(n):
    for j in range(i + 1, n):
        values.append(a[i] ^ a[j])
values.sort()

print(values[k - 1])

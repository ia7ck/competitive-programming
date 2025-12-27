n = int(input())
a = list(map(int, input().split()))
b = list(map(int, input().split()))
c = list(map(int, input().split()))

ans = 0
for i in range(n - 2):
    for j in range(i + 1, n - 1):
        a_sum = sum(a[: i + 1])
        b_sum = sum(b[i + 1 : j + 1])
        c_sum = sum(c[j + 1 :])
        ans = max(ans, a_sum + b_sum + c_sum)
print(ans)

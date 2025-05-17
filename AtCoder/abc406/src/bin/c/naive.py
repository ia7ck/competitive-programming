def check(a):
    if a[0] > a[1]:
        return False
    count_1 = 0
    count_2 = 0
    for i in range(1, len(a) - 1):
        if a[i - 1] < a[i] > a[i + 1]:
            count_1 += 1
        elif a[i - 1] > a[i] < a[i + 1]:
            count_2 += 1
    return count_1 == 1 and count_2 == 1

n = int(input())
p = list(map(int, input().split()))
ans = 0
for i in range(n):
    for j in range(i + 4, n + 1):
        a = p[i:j]
        if check(a):
            ans += 1
print(ans)

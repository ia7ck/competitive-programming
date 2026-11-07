def run(a):
    x = 0
    xs = [x]
    for y in a:
        x += y
        xs.append(x)
    return xs


n = int(input())
a = list(map(int, input().split()))
xs = run(a)
ans = max(xs) - min(xs) + 1
for i in range(n):
    a[i] *= -1
    xs = run(a)
    ans = max(ans, max(xs) - min(xs) + 1)
    a[i] *= -1
print(ans)

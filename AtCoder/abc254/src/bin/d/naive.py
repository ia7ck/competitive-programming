def is_square(x):
    for y in range(1, x + 1):
        if y * y == x:
            return True
    return False

n = int(input())

ans = 0
for i in range(1, n + 1):
    for j in range(1, n + 1):
        if is_square(i * j):
            ans += 1
print(ans)

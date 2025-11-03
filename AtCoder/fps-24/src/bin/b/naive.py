n = int(input())

ans = 0
for a in [0, 1]:
    for b in [0, 1, 2]:
        for c in range(0, n + 1, 2):
            for d in range(0, n + 1, 3):
                if a + b + c + d == n:
                    ans += 1
print(ans)

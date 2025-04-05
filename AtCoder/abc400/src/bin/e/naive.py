def check(n):
    factors = {}
    for i in range(2, n + 1):
        while n % i == 0:
            if i not in factors:
                factors[i] = 0
            factors[i] += 1
            n //= i
    if len(factors) != 2:
        return False
    exps = list(factors.values())
    return exps[0] % 2 == 0 and exps[1] % 2 == 0

q = int(input())
for _ in range(q):
    a = int(input())
    ans = 36
    for n in range(36, a + 1):
        if check(n):
            ans = n
    print(ans)
